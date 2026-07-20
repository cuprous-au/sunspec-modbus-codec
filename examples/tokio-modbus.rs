use heapless::String;
use std::{
    future,
    io::{self},
    net::SocketAddr,
};
use tokio::net::TcpListener;

use sunspec_modbus_codec::{
    ModbusRequest, SunspecModelAdapters, SunspecService,
    sunspec::models::{model_1, model_103},
};
use tokio_modbus::{
    prelude::*,
    server::tcp::{Server, accept_tcp_connection},
};

struct MyInverter {}
impl model_1::ModelAdapter for MyInverter {
    fn manufacturer(&self) -> String<32> {
        String::try_from("Cuprous").unwrap()
    }

    fn model(&self) -> String<32> {
        String::try_from("Inverter 1").unwrap()
    }

    fn serial_number(&self) -> String<32> {
        String::try_from("I-1").unwrap()
    }

    fn options(&self) -> Option<String<16>> {
        Some(String::try_from("opt_a_b_c").unwrap())
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
        let sunspec_service = SunspecService::new(SunspecModelAdapters {
            model_1_adapter: Some(self.inverter),
            model_103_adapter: Some(self.inverter),
        });
        let res = match req {
            Request::ReadHoldingRegisters(addr, cnt) => {
                let mut response_buffer = vec![0_u16; cnt as usize].into_boxed_slice();
                let res = sunspec_service
                    .handle_request(ModbusRequest::ReadRegister(addr, cnt), &mut response_buffer);

                match res {
                    Ok(_) => Ok(Response::ReadHoldingRegisters(response_buffer.into())),
                    _ => Err(ExceptionCode::IllegalFunction),
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
