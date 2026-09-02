use core::ffi::c_void;

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
pub trait ModelSpec<'a> {
    /// The SunSpec model id (e.g. `1` for the common model).
    const MODEL_ID: u16;

    /// This model's read adapter trait. Bounded by `'a` so a request can pass an adapter
    /// that borrows non-`'static` data; the generated impls set it to `dyn ReadAdapter + 'a`.
    type ReadAdapter: ?Sized + 'a;

    /// This model's write adapter trait. Bounded by `'a` for the same reason as
    /// [`ReadAdapter`](ModelSpec::ReadAdapter).
    type WriteAdapter: ?Sized + 'a;

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
    type ReadAdapters<'a>;
    type WriteAdapters<'a>;

    fn map_length(&self) -> u16;

    /// Walk the models in order, encoding each into `buffer` via the cursor.
    fn traverse_read<'a>(
        &self,
        adapters: Self::ReadAdapters<'a>,
        cursor: &mut Cursor<ModbusException>,
        buffer: &mut WritableRegisterBuffer<'a>,
    );

    /// Walk the models in order, decoding `buffer` into each via the cursor.
    ///
    /// The request buffer's lifetime `'buf` is independent of the adapter lifetime `'a`:
    /// the buffer only needs to outlive the call, not the adapters.
    fn traverse_write<'a, 'buf>(
        &self,
        adapters: Self::WriteAdapters<'a>,
        cursor: &mut Cursor<ModbusException>,
        buffer: &ReadableRegisterBuffer<'buf>,
    );
}

/// Advance `cursor` across one model's block, encoding it from `adapter`.
///
/// The block is always consumed (`model_length` words) whether or not it produces data,
/// so the read and write maps stay positionally identical.
pub fn visit_model_read<'a, M: ModelSpec<'a>>(
    cursor: &mut Cursor<ModbusException>,
    buffer: &mut WritableRegisterBuffer<'_>,
    model: &M,
    adapter: &M::ReadAdapter,
) {
    cursor.visit_source_block(model.model_length(), |offset, from, len| {
        model.traverse_points_read(adapter, &mut buffer.slice(from, len), offset)
    });
}

/// Advance `cursor` across one model's block, decoding it into `adapter`.
///
/// A write that lands in a block with no adapter is [`ModbusException::IllegalDataAddress`].
pub fn visit_model_write<'a, M: ModelSpec<'a>>(
    cursor: &mut Cursor<ModbusException>,
    buffer: &ReadableRegisterBuffer<'_>,
    model: &M,
    adapter: &mut M::WriteAdapter,
) {
    cursor.visit_source_block(model.model_length(), |offset, from, len| {
        model.traverse_points_write(adapter, &buffer.slice(from, len), offset)
    });
}

/// Advance `cursor` across one model's block when no read adapter was supplied for it.
///
/// The block is still consumed so the read and write maps stay positionally identical, and
/// the skipped words are filled with `0xffff` (the SunSpec "not implemented" value).
pub fn visit_absent_read<'a, M: ModelSpec<'a>>(
    cursor: &mut Cursor<ModbusException>,
    buffer: &mut WritableRegisterBuffer<'_>,
    model: &M,
) {
    cursor.visit_source_block(model.model_length(), |_, from, len| {
        buffer.slice(from, len).fill(&[0xff, 0xff]);
        Ok(())
    });
}

/// Advance `cursor` across one model's block, rejecting any write that lands in it with
/// [`ModbusException::IllegalDataAddress`].
///
/// Used for models with no writable points, or when no write adapter was supplied.
pub fn reject_model_write<'a, M: ModelSpec<'a>>(cursor: &mut Cursor<ModbusException>, model: &M) {
    cursor.visit_source_block(model.model_length(), |_, _, _| {
        Err(ModbusException::IllegalDataAddress)
    });
}

/// C-FFI dispatch descriptor for one SunSpec model.
///
/// One `#[unsafe(no_mangle)] pub static SUNSPEC_MODEL_<id>: CModel` is generated per model
/// into that model's module. A C `SunspecModelBinding` holds a `*const CModel` pointing at
/// that static, so the `sunspec-modbus-lib-static` service functions dispatch straight
/// through these function pointers with no model-id lookup.
///
/// The `dyn` adapters never cross the C boundary; the concrete `Model<id>{Stateful,Callback}Adapter`
/// pointer is cast back to a reference inside `visit_read` / `visit_write`.
pub struct CModel {
    /// The SunSpec model id this descriptor dispatches, for diagnostics and wire cross-checks.
    pub id: u16,

    /// Register block length in words for the given repeat counts (model header included,
    /// `SunS` excluded). Repeat counts are ignored by non-repeating models.
    pub length: fn(repeat_count_0: u16, repeat_count_1: u16) -> u16,

    /// Decode one model block on a read. `kind`: `1` = stateful adapter pointer, `2` =
    /// callback adapter pointer, anything else = no adapter (the block reads as `0xffff`).
    /// Repeating-group models accept only `kind` `2`.
    ///
    /// # Safety
    /// For `kind` `1` or `2`, `adapter` must point to a live `Model<id>{Stateful,Callback}Adapter`
    /// for this model, valid for the duration of the call.
    pub visit_read: unsafe fn(
        kind: u8,
        adapter: *const c_void,
        repeat_count_0: u16,
        repeat_count_1: u16,
        cursor: &mut Cursor<ModbusException>,
        buffer: &mut WritableRegisterBuffer<'_>,
    ),

    /// Encode one model block on a write. `kind` is as for [`visit_read`](CModel::visit_read).
    /// A non-writable model, or a `kind` with no adapter, rejects the write with
    /// [`ModbusException::IllegalDataAddress`].
    ///
    /// # Safety
    /// As for [`visit_read`](CModel::visit_read), and `adapter` must be uniquely borrowable
    /// for the duration of the call.
    pub visit_write: unsafe fn(
        kind: u8,
        adapter: *mut c_void,
        repeat_count_0: u16,
        repeat_count_1: u16,
        cursor: &mut Cursor<ModbusException>,
        buffer: &ReadableRegisterBuffer<'_>,
    ),
}

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
    pub fn read_registers<'a, B: Into<WritableRegisterBuffer<'a>>>(
        &self,
        address: u16,
        response_buffer: B,
        adapters: L::ReadAdapters<'a>,
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
    pub fn write_multiple_registers<'a, 'buf, B: Into<ReadableRegisterBuffer<'buf>>>(
        &self,
        address: u16,
        request_buffer: B,
        adapters: L::WriteAdapters<'a>,
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
    pub fn write_single_register<'a>(
        &self,
        address: u16,
        value: u16,
        adapters: L::WriteAdapters<'a>,
    ) -> Result<(), ModbusException> {
        self.write_multiple_registers(address, [value].as_slice(), adapters)
    }
}
