pub mod points;

pub mod models;

#[derive(Debug)]
pub enum PointType {
    Uint16,
    Int16,
    Int32,
    Int64,
    Raw16,
    Uint32,
    Uint64,
    Acc16,
    Acc32,
    Acc64,
    Bitfield16,
    Bitfield32,
    Bitfield64,
    Enum16,
    Enum32,
    Float32,
    Float64,
    String,
    Pad,
    Ipaddr,
    Ipv6addr,
    Eui48,
    Sunssf,
    Count,
}

#[derive(Debug)]
pub struct ReadablePoint {
    pub reference: points::PointReference,
    pub size: u16,
    pub data_type: PointType,
    pub writeable: bool,
}
