//! A SunSpec device served over Modbus TCP using `tokio-modbus`
//!
//! ```sh
//! cargo run --example tokio-modbus --features model_103,model_708
//! ```

use std::{
    future,
    io::{self},
    net::SocketAddr,
    sync::atomic::Ordering,
    time::Duration,
};
use sunspec_modbus_lib_rs::{
    ModelList, Sunspec, SunspecConfig,
    sunspec::models::{model_1, model_103, model_708},
};
use tokio::net::TcpListener;

use tokio_modbus::{
    prelude::*,
    server::tcp::{Server, accept_tcp_connection},
};

mod common;

use common::adapters::{
    CURVE_COUNT, CommonModel, CurveModel, InverterModel, POINT_COUNT, VOLTAGE_AN, VOLTAGE_BN,
    VOLTAGE_CN,
};

/// How often [`randomise_voltages`] refreshes the inverter's per-phase voltages.
const VOLTAGE_REFRESH_INTERVAL: Duration = Duration::from_secs(1);

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

/// Stateless: every request reads and writes the global atomics directly, so there's
/// nothing to hold per connection.
struct ExampleService;

impl tokio_modbus::server::Service for ExampleService {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // Zero-sized adapters, built fresh per request.
        // The models they front carry no data of their own, so there's no state here to race across requests.
        let mut common_model = CommonModel;
        let inverter_model = InverterModel;
        let mut curve_model = CurveModel;

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
                        model_708: &curve_model,
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
                    &mut SunspecWriteAdapters {
                        model_1: &mut common_model,
                        model_708: &mut curve_model,
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
                    &mut SunspecWriteAdapters {
                        model_1: &mut common_model,
                        model_708: &mut curve_model,
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
