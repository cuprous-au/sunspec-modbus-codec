use crate::ModbusException;
use crate::buffer::{ReadableRegisterBuffer, WritableRegisterBuffer};
use crate::cursor::{Cursor, CursorResult};

/// SunSpec register maps begin at this Modbus holding-register address.
pub const STARTING_REGISTER_OFFSET: u16 = 40000;

/// The `SunS` identifier that precedes every SunSpec model in the map.
const SUNS_HEADER_WORDS: u16 = 2;

/// A single SunSpec model: how long its register block is, and how to encode/decode its
/// points against a model-specific read or write adapter.
///
/// Generated once per model by `sunspec-gen`. `ReadAdapter` / `WriteAdapter` are that
/// model's own adapter traits; a model with no writable points still has an (empty)
/// `WriteAdapter` and a `traverse_points_write` that rejects every address.
pub trait ModelSpec {
    /// The SunSpec model id (e.g. `1` for the common model).
    const MODEL_ID: u16;

    /// This model's read adapter trait. `'static` because it is named as a `dyn` type in
    /// the per-request adapter tuples; adapters may still borrow `'static` data.
    type ReadAdapter: ?Sized + 'static;

    /// This model's write adapter trait. `'static` for the same reason as
    /// [`ReadAdapter`](ModelSpec::ReadAdapter).
    type WriteAdapter: ?Sized + 'static;

    /// Length of this model's register block, in words, excluding the `SunS` header but
    /// including the model id / length header words. For models with a repeating group
    /// this depends on the repeat count carried by `self`.
    fn model_length(&self) -> u16;

    /// Encode this model's points into `buffer`, starting `offset` words into the model.
    fn traverse_points_read(
        &self,
        adapter: &Self::ReadAdapter,
        buffer: &mut WritableRegisterBuffer<'_>,
        offset: u16,
    ) -> Result<(), ModbusException>;

    /// Decode `buffer` (which starts `offset` words into the model) into `adapter`.
    fn traverse_points_write(
        &self,
        adapter: &mut Self::WriteAdapter,
        buffer: &ReadableRegisterBuffer<'_>,
        offset: u16,
    ) -> Result<(), ModbusException>;
}

/// An ordered, heterogeneous list of [`ModelSpec`]s describing one device's register map.
///
/// Implemented for tuples of 1..=16 models. The single list is the source of truth for
/// layout in both directions; each request supplies the matching tuple of adapters:
///
/// - [`ReadAdapters`](ModelList::ReadAdapters) — one `&ReadAdapter` per model, all required.
/// - [`WriteAdapters`](ModelList::WriteAdapters) — one `Option<&mut WriteAdapter>` per model;
///   a `None` (or a model with no writable points) rejects writes to that block.
pub trait ModelList {
    /// Tuple of `&ReadAdapter` references, one per model, in list order.
    type ReadAdapters<'a>;

    /// Tuple of `Option<&mut WriteAdapter>` references, one per model, in list order.
    type WriteAdapters<'a>;

    /// Total words spanned by every model in the list, excluding the `SunS` header.
    fn map_length(&self) -> u16;

    /// Walk the models in order, encoding each into `buffer` via the cursor.
    fn traverse_read(
        &self,
        adapters: Self::ReadAdapters<'_>,
        cursor: &mut Cursor<ModbusException>,
        buffer: &mut WritableRegisterBuffer<'_>,
    );

    /// Walk the models in order, decoding `buffer` into each via the cursor.
    fn traverse_write(
        &self,
        adapters: Self::WriteAdapters<'_>,
        cursor: &mut Cursor<ModbusException>,
        buffer: &ReadableRegisterBuffer<'_>,
    );
}

/// Advance `cursor` across one model's block, encoding it from `adapter`.
///
/// The block is always consumed (`model_length` words) whether or not it produces data,
/// so the read and write maps stay positionally identical.
#[doc(hidden)]
pub fn visit_model_read<M: ModelSpec>(
    cursor: &mut Cursor<ModbusException>,
    buffer: &mut WritableRegisterBuffer<'_>,
    model: &M,
    adapter: &M::ReadAdapter,
) {
    let _ = cursor.visit_source_block(model.model_length(), |offset, from, len| {
        model.traverse_points_read(adapter, &mut buffer.slice(from, len), offset)
    });
}

/// Advance `cursor` across one model's block, decoding it into `adapter`.
///
/// A write that lands in a block with no adapter is [`ModbusException::IllegalDataAddress`].
#[doc(hidden)]
pub fn visit_model_write<M: ModelSpec>(
    cursor: &mut Cursor<ModbusException>,
    buffer: &ReadableRegisterBuffer<'_>,
    model: &M,
    adapter: Option<&mut M::WriteAdapter>,
) {
    let _ = cursor.visit_source_block(model.model_length(), |offset, from, len| match adapter {
        Some(adapter) => model.traverse_points_write(adapter, &buffer.slice(from, len), offset),
        None => Err(ModbusException::IllegalDataAddress),
    });
}

macro_rules! impl_model_list {
    ($($model:ident $index:tt),+) => {
        impl<$($model: ModelSpec),+> ModelList for ($($model,)+) {
            type ReadAdapters<'a> = ($(&'a <$model as ModelSpec>::ReadAdapter,)+);
            type WriteAdapters<'a> = ($(Option<&'a mut <$model as ModelSpec>::WriteAdapter>,)+);

            fn map_length(&self) -> u16 {
                0 $(+ self.$index.model_length())+
            }

            fn traverse_read(
                &self,
                adapters: Self::ReadAdapters<'_>,
                cursor: &mut Cursor<ModbusException>,
                buffer: &mut WritableRegisterBuffer<'_>,
            ) {
                $( visit_model_read(cursor, buffer, &self.$index, adapters.$index); )+
            }

            fn traverse_write(
                &self,
                adapters: Self::WriteAdapters<'_>,
                cursor: &mut Cursor<ModbusException>,
                buffer: &ReadableRegisterBuffer<'_>,
            ) {
                $( visit_model_write(cursor, buffer, &self.$index, adapters.$index); )+
            }
        }
    };
}

impl_model_list!(M0 0);
impl_model_list!(M0 0, M1 1);
impl_model_list!(M0 0, M1 1, M2 2);
impl_model_list!(M0 0, M1 1, M2 2, M3 3);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6, M7 7);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6, M7 7, M8 8);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6, M7 7, M8 8, M9 9);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6, M7 7, M8 8, M9 9, M10 10);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6, M7 7, M8 8, M9 9, M10 10, M11 11);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6, M7 7, M8 8, M9 9, M10 10, M11 11, M12 12);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6, M7 7, M8 8, M9 9, M10 10, M11 11, M12 12, M13 13);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6, M7 7, M8 8, M9 9, M10 10, M11 11, M12 12, M13 13, M14 14);
impl_model_list!(M0 0, M1 1, M2 2, M3 3, M4 4, M5 5, M6 6, M7 7, M8 8, M9 9, M10 10, M11 11, M12 12, M13 13, M14 14, M15 15);

/// A SunSpec register-map codec bound to a fixed [`ModelList`].
///
/// Build one from the ordered models the device exposes, then serve Modbus reads and
/// writes against it, passing the matching adapter tuple per request:
///
/// ```ignore
/// static SUNSPEC: Sunspec<(model_1::Model1, model_103::Model103)> =
///     Sunspec::new((model_1::Model1, model_103::Model103));
///
/// SUNSPEC.read_registers(addr, &mut buf[..], (&common, &inverter))?;
/// SUNSPEC.write_multiple_registers(addr, req, (Some(&mut common), None))?;
/// ```
pub struct Sunspec<L> {
    models: L,
}

impl<L: ModelList> Sunspec<L> {
    /// Bind the codec to `models`. The list is fixed for the life of the value and is
    /// used identically for reads and writes.
    pub const fn new(models: L) -> Self {
        Self { models }
    }

    /// The models this codec serves.
    pub fn models(&self) -> &L {
        &self.models
    }

    /// Encode a holding-register read of `response_buffer.len()` words starting at
    /// `address` into `response_buffer`. Registers past the end of the model map are
    /// filled with `0xffff`.
    pub fn read_registers<'b, B: Into<WritableRegisterBuffer<'b>>>(
        &self,
        address: u16,
        response_buffer: B,
        adapters: L::ReadAdapters<'_>,
    ) -> Result<(), ModbusException> {
        let mut buffer = response_buffer.into();
        let count = buffer.len();

        if address < STARTING_REGISTER_OFFSET || address > u16::MAX - count {
            return Err(ModbusException::IllegalDataAddress);
        }

        let mut cursor: Cursor<ModbusException> =
            Cursor::new(address - STARTING_REGISTER_OFFSET, count);

        let _ = cursor.visit_source_block(SUNS_HEADER_WORDS, |offset, from, len| {
            buffer.slice(from, len).write_string(c"SunS", offset);
            Ok(())
        });

        self.models
            .traverse_read(adapters, &mut cursor, &mut buffer);

        match cursor.result() {
            CursorResult::Error(exception) => Err(exception),
            CursorResult::Incomplete(remainder) => {
                buffer
                    .slice(remainder, count - remainder)
                    .fill(&[0xff, 0xff]);
                Ok(())
            }
            CursorResult::Complete => Ok(()),
        }
    }

    /// Decode a write of `request_buffer.len()` words starting at `address` into the
    /// relevant models. A write that touches a block whose `Option` adapter is `None`,
    /// or a model with no writable points, is rejected.
    pub fn write_multiple_registers<'b, B: Into<ReadableRegisterBuffer<'b>>>(
        &self,
        address: u16,
        request_buffer: B,
        adapters: L::WriteAdapters<'_>,
    ) -> Result<(), ModbusException> {
        let buffer = request_buffer.into();
        let count = buffer.len();

        if address < STARTING_REGISTER_OFFSET || address > u16::MAX - count {
            return Err(ModbusException::IllegalDataAddress);
        }

        let mut cursor: Cursor<ModbusException> =
            Cursor::new(address - STARTING_REGISTER_OFFSET, count);

        let _ = cursor.visit_source_block(SUNS_HEADER_WORDS, |_, _, _| Ok(()));

        self.models.traverse_write(adapters, &mut cursor, &buffer);

        match cursor.result() {
            CursorResult::Error(exception) => Err(exception),
            CursorResult::Incomplete(_) | CursorResult::Complete => Ok(()),
        }
    }

    /// Decode a single-register write. Equivalent to [`write_multiple_registers`] with a
    /// one-word buffer.
    ///
    /// [`write_multiple_registers`]: Sunspec::write_multiple_registers
    pub fn write_single_register(
        &self,
        address: u16,
        value: u16,
        adapters: L::WriteAdapters<'_>,
    ) -> Result<(), ModbusException> {
        let words = [value];
        self.write_multiple_registers(address, &words[..], adapters)
    }
}
