use std::{
    ffi::CStr,
    future,
    io::{self},
    net::SocketAddr,
};
use sunspec_modbus_lib_rs::{
    ModbusRequest, handle_request,
    sunspec::{
        adapters::SunspecAdapters,
        models::{model_1, model_103},
    },
};
use tokio::net::TcpListener;

use tokio_modbus::{
    prelude::*,
    server::tcp::{Server, accept_tcp_connection},
};

struct MyInverter {}
impl model_1::ModelAdapter for MyInverter {
    fn manufacturer(&self) -> &CStr {
        &c"Cuprous"
    }

    fn model(&self) -> &CStr {
        &c"Inverter 1"
    }

    fn serial_number(&self) -> &CStr {
        &c"I-1"
    }

    fn options(&self) -> Option<&CStr> {
        Some(&c"opt_a_b_c")
    }
}

impl model_103::ModelAdapter for MyInverter {
    fn amps(&self) -> u16 {
        102
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

    fn a_sf(&self) -> u16 {
        1
    }

    fn phase_voltage_an(&self) -> u16 {
        1
    }

    fn phase_voltage_bn(&self) -> u16 {
        1
    }

    fn phase_voltage_cn(&self) -> u16 {
        1
    }

    fn v_sf(&self) -> u16 {
        1
    }

    fn watts(&self) -> i16 {
        1
    }

    fn w_sf(&self) -> u16 {
        1
    }

    fn hz(&self) -> u16 {
        1
    }

    fn hz_sf(&self) -> u16 {
        1
    }

    fn watt_hours(&self) -> u32 {
        1
    }

    fn wh_sf(&self) -> u16 {
        1
    }

    fn cabinet_temperature(&self) -> i16 {
        1
    }

    fn tmp_sf(&self) -> u16 {
        1
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

struct ExampleService<'a> {
    inverter: &'a MyInverter,
}

impl<'a> tokio_modbus::server::Service for ExampleService<'a> {
    type Request = Request<'a>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let adapters = SunspecAdapters {
            model_1_adapter: Some(self.inverter),
            model_103_adapter: Some(self.inverter),
            ..Default::default()
        };
        let res = match req {
            Request::ReadHoldingRegisters(addr, cnt) => {
                println!("{} -> {}", addr, cnt);
                let mut response_buffer = vec![0_u16; cnt as usize].into_boxed_slice();
                let res = handle_request(
                    &adapters,
                    ModbusRequest::ReadRegister(addr, cnt),
                    &mut response_buffer as &mut [u16],
                );

                for word in &response_buffer {
                    print!("{:x} ", word);
                }
                println!(";");

                Ok(Response::ReadHoldingRegisters(response_buffer.into()))
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

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let socket_addr = "127.0.0.1:5502".parse().unwrap();

    server_context(socket_addr).await
}

async fn server_context(socket_addr: SocketAddr) -> io::Result<()> {
    println!("Starting up server on {socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    let server = Server::new(listener);

    let inverter = &MyInverter {};

    let new_service = |_socket_addr| Ok(Some(ExampleService { inverter: inverter }));
    let on_connected = |stream, socket_addr| async move {
        accept_tcp_connection(stream, socket_addr, new_service)
    };
    let on_process_error = |err| {
        eprintln!("{err}");
    };
    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}
