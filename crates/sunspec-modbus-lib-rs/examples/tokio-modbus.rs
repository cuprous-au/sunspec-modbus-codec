use std::{
    cell::RefCell,
    ffi::CStr,
    future,
    io::{self},
    net::SocketAddr,
    sync::atomic::{AtomicU16, Ordering},
    time::Duration,
};
use sunspec_modbus_lib_rs::{
    ModelList, Sunspec,
    sunspec::{
        adapters::{ReadBinding, WriteBinding},
        models::{model_1, model_103},
    },
};
use tokio::net::TcpListener;

use tokio_modbus::{
    prelude::*,
    server::tcp::{Server, accept_tcp_connection},
};

/// How often [`randomise_voltages`] refreshes the inverter's per-phase voltages.
const VOLTAGE_REFRESH_INTERVAL: Duration = Duration::from_secs(1);

/// The device's entire mutable state. Everything else served below (manufacturer strings,
/// amperage, frequency, ...) is fixed, so there's nothing else to hold.
static DEVICE_ADDRESS: AtomicU16 = AtomicU16::new(0);
static VOLTAGE_AN: AtomicU16 = AtomicU16::new(0);
static VOLTAGE_BN: AtomicU16 = AtomicU16::new(0);
static VOLTAGE_CN: AtomicU16 = AtomicU16::new(0);

/// Backs `model_1`'s adapters. Zero-sized: its one piece of state, the device address,
/// lives in [`DEVICE_ADDRESS`] rather than a struct field.
struct CommonModel;

impl model_1::ReadAdapter for CommonModel {
    fn manufacturer(&self) -> &CStr {
        c"Cuprous"
    }

    fn model(&self) -> &CStr {
        c"Inverter 1"
    }

    fn options(&self) -> Option<&CStr> {
        Some(c"opt_a_b_c")
    }

    fn version(&self) -> Option<&CStr> {
        Some(c"v0.1")
    }

    fn serial_number(&self) -> &CStr {
        c"I-1"
    }

    fn device_address(&self) -> Option<u16> {
        Some(DEVICE_ADDRESS.load(Ordering::Relaxed))
    }
}

impl model_1::WriteAdapter for CommonModel {
    fn set_device_address(&mut self, value: u16) {
        DEVICE_ADDRESS.store(value, Ordering::Relaxed);
    }
}

/// Backs `model_103`'s read adapter. Zero-sized: its mutable state, the per-phase
/// voltages, lives in [`VOLTAGE_AN`] / [`VOLTAGE_BN`] / [`VOLTAGE_CN`] rather than a
/// struct field, refreshed on a timer by [`randomise_voltages`].
struct InverterModel;

impl model_103::ReadAdapter for InverterModel {
    fn amps(&self) -> u16 {
        0
    }

    fn amps_phase_a(&self) -> u16 {
        1
    }

    fn amps_phase_b(&self) -> u16 {
        1
    }

    fn amps_phase_c(&self) -> u16 {
        1
    }

    fn a_sf(&self) -> i16 {
        -1
    }

    fn phase_voltage_an(&self) -> u16 {
        VOLTAGE_AN.load(Ordering::Relaxed)
    }

    fn phase_voltage_bn(&self) -> u16 {
        VOLTAGE_BN.load(Ordering::Relaxed)
    }

    fn phase_voltage_cn(&self) -> u16 {
        VOLTAGE_CN.load(Ordering::Relaxed)
    }

    fn v_sf(&self) -> i16 {
        -1
    }

    fn watts(&self) -> i16 {
        1
    }

    fn w_sf(&self) -> i16 {
        1
    }

    fn hz(&self) -> u16 {
        1234
    }

    fn hz_sf(&self) -> i16 {
        -2
    }

    fn watt_hours(&self) -> u32 {
        1
    }

    fn wh_sf(&self) -> i16 {
        0
    }

    fn cabinet_temperature(&self) -> i16 {
        1
    }

    fn tmp_sf(&self) -> i16 {
        0
    }

    fn operating_state(&self) -> model_103::St {
        model_103::St::Standby
    }

    fn event1(&self) -> u32 {
        1
    }

    fn event_bitfield_2(&self) -> u32 {
        1
    }
}

struct SunspecModel {
    model_1: model_1::Model1,
    model_103: model_103::Model103,
}

struct SunspecReadAdapters<'a> {
    model_1: &'a RefCell<dyn model_1::ReadAdapter>,
    model_103: &'a RefCell<dyn model_103::ReadAdapter>,
}

struct ReadAdapterIter<'a> {
    models: &'a SunspecModel,
    adapters: &'a SunspecReadAdapters<'a>,
    state: usize,
}

impl<'a> Iterator for ReadAdapterIter<'a> {
    type Item = ReadBinding<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.state {
            0 => Some(ReadBinding::Model1(
                &self.models.model_1,
                self.adapters.model_1.borrow(),
            )),
            1 => Some(ReadBinding::Model103(
                &self.models.model_103,
                self.adapters.model_103.borrow(),
            )),
            _ => None,
        };
        self.state += 1;
        result
    }
}

struct SunspecWriteAdapters<'a> {
    model_1: &'a RefCell<dyn model_1::WriteAdapter>,
}

struct WriteAdapterIter<'a> {
    models: &'a SunspecModel,
    adapters: &'a SunspecWriteAdapters<'a>,
    state: usize,
}

impl<'a> Iterator for WriteAdapterIter<'a> {
    type Item = WriteBinding<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.state {
            0 => Some(WriteBinding::Model1(
                &self.models.model_1,
                self.adapters.model_1.borrow_mut(),
            )),
            1 => Some(WriteBinding::Model103(&self.models.model_103)),
            _ => None,
        };
        self.state += 1;
        result
    }
}

impl ModelList for SunspecModel {
    type ReadAdapters<'a> = &'a SunspecReadAdapters<'a>;

    type WriteAdapters<'a> = &'a SunspecWriteAdapters<'a>;

    fn read_iter<'a>(
        &'a self,
        adapters: Self::ReadAdapters<'a>,
    ) -> impl Iterator<Item = ReadBinding<'a>> {
        ReadAdapterIter {
            models: self,
            adapters,
            state: 0,
        }
    }

    fn write_iter<'a>(
        &'a self,
        adapters: Self::WriteAdapters<'a>,
    ) -> impl Iterator<Item = WriteBinding<'a>> {
        WriteAdapterIter {
            models: self,
            adapters,
            state: 0,
        }
    }
}

/// The device's register map: the common model followed by an inverter model. The same
/// list backs both reads and writes.
const SUNSPEC: Sunspec<SunspecModel> = Sunspec::new(SunspecModel {
    model_1: model_1::Model1,
    model_103: model_103::Model103,
});

/// Stateless: every request reads and writes the global atomics directly, so there's
/// nothing to hold per connection.
struct ExampleService;

impl tokio_modbus::server::Service for ExampleService {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // `RefCell`s over zero-sized adapters, built fresh per request purely to satisfy
        // `ReadBinding`/`WriteBinding`'s borrow-guard types — the models they front carry no
        // data of their own, so there's no state here to race across requests.
        let common_model = RefCell::new(CommonModel);
        let inverter_model = RefCell::new(InverterModel);

        println!("Handling {req:?}");
        let res = match req {
            Request::ReadHoldingRegisters(addr, cnt) => {
                println!("{} -> {} ({} words)", addr, addr + cnt, cnt);

                let mut response_buffer = vec![0_u16; cnt as usize].into_boxed_slice();
                match SUNSPEC.read_registers(
                    addr,
                    &mut response_buffer as &mut [u16],
                    &SunspecReadAdapters {
                        model_1: &common_model,
                        model_103: &inverter_model,
                    },
                ) {
                    Ok(_) => {
                        for word in &response_buffer {
                            print!("{:x} ", word);
                        }
                        println!(";");

                        Ok(Response::ReadHoldingRegisters(response_buffer.into()))
                    }
                    Err(code) => Err(ExceptionCode::new(code as u8)),
                }
            }
            Request::WriteMultipleRegisters(addr, buffer) => {
                let len = buffer.len() as u16;
                match SUNSPEC.write_multiple_registers(
                    addr,
                    buffer.as_ref(),
                    &SunspecWriteAdapters {
                        model_1: &common_model,
                    },
                ) {
                    Ok(_) => Ok(Response::WriteMultipleRegisters(addr, len)),
                    Err(code) => Err(ExceptionCode::new(code as u8)),
                }
            }
            Request::WriteSingleRegister(addr, value) => {
                match SUNSPEC.write_single_register(
                    addr,
                    value,
                    &SunspecWriteAdapters {
                        model_1: &common_model,
                    },
                ) {
                    Ok(_) => Ok(Response::WriteSingleRegister(addr, value)),
                    Err(code) => Err(ExceptionCode::new(code as u8)),
                }
            }
            _ => {
                println!(
                    "SERVER: Exception::IllegalFunction - Unimplemented function code in request: {req:?}"
                );
                Err(ExceptionCode::IllegalFunction)
            }
        };
        future::ready(res)
    }
}

/// Refreshes the inverter's per-phase voltages with new random values every
/// [`VOLTAGE_REFRESH_INTERVAL`], for as long as the server runs.
async fn randomise_voltages() {
    let mut ticker = tokio::time::interval(VOLTAGE_REFRESH_INTERVAL);
    loop {
        ticker.tick().await;
        VOLTAGE_AN.store(rand::random_range(2300..2500), Ordering::Relaxed);
        VOLTAGE_BN.store(rand::random_range(2300..2500), Ordering::Relaxed);
        VOLTAGE_CN.store(rand::random_range(2300..2500), Ordering::Relaxed);
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let socket_addr = "127.0.0.1:5502"
        .parse()
        .expect("Failed to parse socked address");

    server_context(socket_addr).await
}

async fn server_context(socket_addr: SocketAddr) -> io::Result<()> {
    println!("Starting up server on {socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    let server = Server::new(listener);

    tokio::spawn(randomise_voltages());

    let new_service = |_socket_addr| Ok(Some(ExampleService));

    let on_connected = |stream, socket_addr| async move {
        accept_tcp_connection(stream, socket_addr, new_service)
    };
    let on_process_error = |err| {
        eprintln!("{err}");
    };
    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}
