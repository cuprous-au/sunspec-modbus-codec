use std::{
    future,
    io::{self},
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use sunspec_modbus_lib_rs::{
    ModbusRequest, c_char_array, handle_request,
    sunspec::{
        adapters::SunspecAdapters,
        models::{model_1::Model1StatefulAdapter, model_103},
    },
};
use tokio::net::TcpListener;

use tokio_modbus::{
    prelude::*,
    server::tcp::{Server, accept_tcp_connection},
};

struct InverterModel {
    pub amp_value: u16,
    pub voltages: [u16; 3],
}
impl model_103::ModelAdapter for InverterModel {
    fn amps(&self) -> u16 {
        self.amp_value
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
        self.voltages[0]
    }

    fn phase_voltage_bn(&self) -> u16 {
        self.voltages[1]
    }

    fn phase_voltage_cn(&self) -> u16 {
        self.voltages[2]
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

struct ExampleService {
    common_model: Arc<Mutex<Model1StatefulAdapter>>,
    inverter: Arc<Mutex<InverterModel>>,
}

impl tokio_modbus::server::Service for ExampleService {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut common_model = self
            .common_model
            .lock()
            .expect("Failed to get mutable reference to common model");
        let mut inverter_model = self
            .inverter
            .lock()
            .expect("Failed to get mutable reference to inverter model");

        inverter_model.voltages = [
            rand::random_range(2300..2500),
            rand::random_range(2300..2500),
            rand::random_range(2300..2500),
        ];
        let mut adapters = SunspecAdapters {
            model_1_adapter: Some(&mut *common_model),
            model_103_adapter: Some(&mut *inverter_model),
            ..Default::default()
        };
        println!("Handling {req:?}");
        let res = match req {
            Request::ReadHoldingRegisters(addr, cnt) => {
                println!("{} -> {} ({} words)", addr, addr + cnt, cnt);

                let mut response_buffer = vec![0_u16; cnt as usize].into_boxed_slice();
                match handle_request(
                    &mut adapters,
                    ModbusRequest::ReadRegister(addr, cnt),
                    &mut response_buffer as &mut [u16],
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
            Request::WriteMultipleRegisters(addr, mut buffer) => {
                let buf = buffer.to_mut();
                let mut slice = buf.clone().into_boxed_slice();
                match handle_request(
                    &mut adapters,
                    ModbusRequest::WriteMultipleRegisters(addr, buf.len() as u16),
                    &mut slice as &mut [u16],
                ) {
                    Ok(_) => {
                        Ok(Response::WriteMultipleRegisters(addr, buffer.len() as u16))
                    }
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

    let common_model = Arc::new(Mutex::new(Model1StatefulAdapter {
        manufacturer: c_char_array!("Cuprous"),
        model: c_char_array!("Inverter 1"),
        serial_number: c_char_array!("I-1"),
        options: c_char_array!("opt_a_b_c"),
        version: c_char_array!("v0.1"),
        device_address: 0,
    }));

    let inverter = Arc::new(Mutex::new(InverterModel {
        amp_value: 0,
        voltages: [0, 0, 0],
    }));

    let new_service = |_socket_addr| {
        Ok(Some(ExampleService {
            common_model: common_model.clone(),
            inverter: inverter.clone(),
        }))
    };

    let on_connected = |stream, socket_addr| async move {
        accept_tcp_connection(stream, socket_addr, new_service)
    };
    let on_process_error = |err| {
        eprintln!("{err}");
    };
    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}
