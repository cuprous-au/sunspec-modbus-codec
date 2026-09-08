use std::{
    cell::RefCell,
    future,
    io::{self},
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use sunspec_modbus_lib_rs::{
    ModelList, Sunspec, c_char_array,
    sunspec::{
        adapters::{ReadBinding, WriteBinding},
        models::{
            model_1::{self, Model1StatefulAdapter},
            model_103,
        },
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
impl model_103::ReadAdapter for InverterModel {
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

struct SunspecModel {
    model_1: model_1::Model1,
    model_103: model_103::Model103,
}

struct SunspecReadAdapters<'a> {
    model_1: &'a dyn model_1::ReadAdapter,
    model_103: &'a dyn model_103::ReadAdapter,
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
                self.adapters.model_1,
            )),
            1 => Some(ReadBinding::Model103(
                &self.models.model_103,
                self.adapters.model_103,
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

struct ExampleService {
    common_model: Arc<Mutex<RefCell<Model1StatefulAdapter>>>,
    inverter: Arc<Mutex<RefCell<InverterModel>>>,
}

impl tokio_modbus::server::Service for ExampleService {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let common_model = self
            .common_model
            .lock()
            .expect("Failed to get mutable reference to common model");
        let inverter_model = self
            .inverter
            .lock()
            .expect("Failed to get mutable reference to inverter model");

        inverter_model.borrow_mut().voltages = [
            rand::random_range(2300..2500),
            rand::random_range(2300..2500),
            rand::random_range(2300..2500),
        ];

        println!("Handling {req:?}");
        let res = match req {
            Request::ReadHoldingRegisters(addr, cnt) => {
                println!("{} -> {} ({} words)", addr, addr + cnt, cnt);

                let mut response_buffer = vec![0_u16; cnt as usize].into_boxed_slice();
                match SUNSPEC.read_registers(
                    addr,
                    &mut response_buffer as &mut [u16],
                    &SunspecReadAdapters {
                        model_1: &*common_model.borrow(),
                        model_103: &*inverter_model.borrow(),
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
                        model_1: &*common_model,
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
                        model_1: &*common_model,
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

    let common_model = Arc::new(Mutex::new(RefCell::new(Model1StatefulAdapter {
        manufacturer: c_char_array!("Cuprous"),
        model: c_char_array!("Inverter 1"),
        serial_number: c_char_array!("I-1"),
        options: c_char_array!("opt_a_b_c"),
        version: c_char_array!("v0.1"),
        device_address: 0,
    })));

    let inverter = Arc::new(Mutex::new(RefCell::new(InverterModel {
        amp_value: 0,
        voltages: [0, 0, 0],
    })));

    let new_service = |_socket_addr|{
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
