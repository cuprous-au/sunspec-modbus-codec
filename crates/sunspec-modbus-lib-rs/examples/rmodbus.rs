//! A SunSpec device served over Modbus TCP by `rmodbus` frame handling on an `embassy-net` stack.
//!
//! Everything except [`main`] is `core`-only and allocation-free, so it runs unchanged on a
//! microcontroller: swap the tap device and `arch-std` executor in [`main`] for your board's
//! Ethernet driver and executor. Run on a host with a tap interface configured as the gateway:
//!
//! ```sh
//! sudo ip tuntap add name tap0 mode tap user $USER
//! sudo ip addr add 192.168.69.1/24 dev tap0
//! sudo ip link set tap0 up
//! cargo run --example rmodbus --features model_103,model_708
//! ```

use core::sync::atomic::Ordering;

use embassy_executor::Spawner;
use embassy_net::{Ipv4Address, Ipv4Cidr, Stack, StackResources, StaticConfigV4, tcp::TcpSocket};
use embassy_net_tuntap::TunTapDevice;
use embassy_time::{Duration, Ticker};
use embedded_io_async::Write as _;
use rmodbus::{
    ErrorKind, ModbusFrameBuf, ModbusProto,
    server::{ModbusFrame, Read, Write},
};
use static_cell::StaticCell;
use sunspec_modbus_lib_rs::{
    ModbusException, ModelList, Sunspec, SunspecConfig,
    sunspec::models::{model_1, model_103, model_708},
};

mod common;

use common::adapters::{
    CURVE_COUNT, CommonModel, CurveModel, InverterModel, POINT_COUNT, VOLTAGE_AN, VOLTAGE_BN,
    VOLTAGE_CN,
};

/// How often [`randomise_voltages`] refreshes the inverter's per-phase voltages.
const VOLTAGE_REFRESH_INTERVAL: Duration = Duration::from_secs(1);

const MODBUS_PORT: u16 = 502;
const UNIT_ID: u8 = 1;

#[derive(ModelList)]
struct SunspecModels {
    model_1: model_1::Model1,
    model_103: model_103::Model103,
    model_708: model_708::Model708,
}

/// The device's register map: the common model, an inverter model, and a DER high-voltage-trip
/// curve model. The same list backs both reads and writes.
const SUNSPEC: Sunspec<SunspecModels> = Sunspec::new(
    SunspecModels {
        model_1: model_1::Model1,
        model_103: model_103::Model103,
        model_708: model_708::Model708 {
            stored_curve_count: CURVE_COUNT,
            number_of_points: POINT_COUNT,
        },
    },
    SunspecConfig::DEFAULT,
);

/// Maps a SunSpec exception onto the equivalent `rmodbus` error, which
/// [`ModbusFrame::process_external_read`]/[`process_external_write`](ModbusFrame::process_external_write)
/// turn into an exception response.
fn to_error_kind(exception: ModbusException) -> ErrorKind {
    match exception {
        ModbusException::IllegalFunction => ErrorKind::IllegalFunction,
        ModbusException::IllegalDataAddress => ErrorKind::IllegalDataAddress,
        ModbusException::IllegalDataValue => ErrorKind::IllegalDataValue,
        ModbusException::ServerDeviceFailure => ErrorKind::SlaveDeviceFailure,
        ModbusException::Acknowledge => ErrorKind::Acknowledge,
        ModbusException::ServerDeviceBusy => ErrorKind::SlaveDeviceBusy,
        ModbusException::MemoryParityError => ErrorKind::MemoryParityError,
        ModbusException::GatewayPathUnavailable => ErrorKind::GatewayPathUnavailable,
        ModbusException::GatewayTargetDevice => ErrorKind::GatewayTargetFailed,
    }
}

/// Serves one Modbus TCP request: parses `request`, reads or writes the SunSpec register map,
/// and returns the encoded response, if one is due.
fn handle_request(request: &[u8]) -> Result<Option<heapless::Vec<u8, 256>>, ErrorKind> {
    // Zero-sized adapters, built fresh per request.
    // The models they front carry no data of their own, so there's no state here to race across requests.
    let mut common_model = CommonModel;
    let inverter_model = InverterModel;
    let mut curve_model = CurveModel;

    let mut response = heapless::Vec::new();
    let mut frame = ModbusFrame::new(UNIT_ID, request, ModbusProto::TcpUdp, &mut response);
    frame.parse()?;

    if frame.processing_required {
        if frame.readonly {
            let result = match frame.get_external_read()? {
                Read::Words(read) => SUNSPEC
                    .read_registers(
                        read.address,
                        read.buf,
                        &SunspecReadAdapters {
                            model_1: &common_model,
                            model_103: &inverter_model,
                            model_708: &curve_model,
                        },
                    )
                    .map_err(to_error_kind),
                Read::Bits(_) => Err(ErrorKind::IllegalFunction),
            };
            frame.process_external_read(result)?;
        } else {
            let mut adapters = SunspecWriteAdapters {
                model_1: &mut common_model,
                model_708: &mut curve_model,
            };
            let result = match frame.get_external_write()? {
                Write::Words(write) => {
                    SUNSPEC.write_multiple_registers(write.address, write.data, &mut adapters)
                }
                Write::Bits(_) => Err(ModbusException::IllegalFunction),
            }
            .map_err(to_error_kind);
            frame.process_external_write(result)?;
        }
    }

    if frame.response_required {
        frame.finalize_response()?;
        Ok(Some(response))
    } else {
        Ok(None)
    }
}

/// Number of bytes of a Modbus TCP ADU that precede the length field's own count: the
/// transaction identifier, protocol identifier and length field itself.
const MBAP_LENGTH_OFFSET: usize = 6;

/// Returns the total ADU length (MBAP header, including the unit identifier, plus PDU) once
/// enough of `buf` has arrived to read the MBAP length field, per the Modbus TCP framing in the
/// spec (unlike serial Modbus, TCP has no inter-frame silence to mark boundaries, so this length
/// field is the only way to tell where one request ends and the next begins).
fn mbap_frame_len(buf: &[u8]) -> Option<usize> {
    let header = buf.get(..MBAP_LENGTH_OFFSET)?;
    let length_field = u16::from_be_bytes([header[4], header[5]]) as usize;
    Some(MBAP_LENGTH_OFFSET + length_field)
}

/// Accepts one connection at a time on [`MODBUS_PORT`], answering requests until the client
/// disconnects or sends something unparseable.
#[embassy_executor::task]
async fn modbus_server(stack: Stack<'static>) {
    let mut rx_buffer = [0; 512];
    let mut tx_buffer = [0; 512];
    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

    loop {
        if socket.accept(MODBUS_PORT).await.is_err() {
            continue;
        }

        let mut buf: ModbusFrameBuf = [0; 256];
        let mut len = 0;
        'connection: while let Ok(n) = socket.read(&mut buf[len..]).await
            && n > 0
        {
            len += n;

            // Drain every complete frame already buffered before reading more: a single read can
            // return several coalesced requests at once.
            while let Some(frame_len) = mbap_frame_len(&buf[..len]) {
                if frame_len > buf.len() {
                    // Declares more data than the buffer, and so more than rmodbus supports.
                    break 'connection;
                }
                if frame_len > len {
                    // Frame isn't fully buffered yet.
                    break;
                }

                match handle_request(&buf[..frame_len]) {
                    Ok(Some(response)) => {
                        if socket.write_all(&response).await.is_err() {
                            break 'connection;
                        }
                    }
                    Ok(None) => {}
                    Err(_) => break 'connection,
                }

                // Advance to the next frame
                buf.copy_within(frame_len..len, 0);
                len -= frame_len;
            }
        }
        socket.abort();
        let _ = socket.flush().await;
    }
}

/// Refreshes the inverter's per-phase voltages with new pseudo-random values every
/// [`VOLTAGE_REFRESH_INTERVAL`], for as long as the server runs.
#[embassy_executor::task]
async fn randomise_voltages() {
    let mut ticker = Ticker::every(VOLTAGE_REFRESH_INTERVAL);
    let mut state: u32 = 0x2545_f491;
    let mut next_voltage = move || {
        // xorshift32, mapped onto 2300..2500.
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        2300 + (state % 200) as u16
    };
    loop {
        ticker.next().await;
        VOLTAGE_AN.store(next_voltage(), Ordering::Relaxed);
        VOLTAGE_BN.store(next_voltage(), Ordering::Relaxed);
        VOLTAGE_CN.store(next_voltage(), Ordering::Relaxed);
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, TunTapDevice>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let device = TunTapDevice::new("tap0").expect("Failed to open tap0");
    let config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 69, 2), 24),
        gateway: Some(Ipv4Address::new(192, 168, 69, 1)),
        dns_servers: heapless::Vec::new(),
    });

    static RESOURCES: StaticCell<StackResources<2>> = StaticCell::new();
    let (stack, runner) =
        embassy_net::new(device, config, RESOURCES.init(StackResources::new()), 0);

    spawner.spawn(net_task(runner).unwrap());
    spawner.spawn(randomise_voltages().unwrap());
    spawner.spawn(modbus_server(stack).unwrap());
}
