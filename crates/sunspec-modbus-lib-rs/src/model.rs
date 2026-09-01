use crate::ModbusException;
use crate::buffer::{ReadableRegisterBuffer, WritableRegisterBuffer};
use crate::sunspec::models::model_703;


pub trait Model<ReadAdapter: ?Sized, WriteAdapter: ?Sized> {
    fn model_length(&self) -> u16;

    fn traverse_points_read<'a>(
        &self,
        model: &ReadAdapter,
        buffer: &mut WritableRegisterBuffer<'a>,
        offset: u16,
    ) -> Result<(), ModbusException>;

    fn traverse_points_write<'a>(
        &self,
        model: &mut WriteAdapter,
        buffer: &ReadableRegisterBuffer<'a>,
        offset: u16,
    ) -> Result<(), ModbusException>;
}
