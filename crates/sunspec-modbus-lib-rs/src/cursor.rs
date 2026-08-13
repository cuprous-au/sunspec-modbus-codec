/// A mutable handle to support stepping through a u16 indexed target for some range of values.
///
/// Once the target has been fully traversed, further calls are treated as a no-op, short-circuiting any unnecessary
/// logic.
pub struct Cursor {
    /// Number of words to skip from the source
    ///
    /// Any blocks of data that fall before this offset are skipped entirely, and these handlers are not invoked at all.
    /// The handler for the first block after the offset will be invoked with the remaining offset, and the handler is
    /// expected to skip those words.
    pub source_offset: u16,
    /// The current offset within the target, unless it has already been fully traversed.
    pub target_offset: Option<u16>,
    /// The total number of words to be traversed
    pub limit: u16,
}

impl Cursor {
    pub fn new(source_offset: u16, limit: u16) -> Self {
        Self {
            source_offset,
            target_offset: if limit > 0 { Some(0) } else { None },
            limit,
        }
    }

    /// Provide a handler that represents a fixed size source block.
    ///
    /// - If the target has already been fully traversed, this is a no-op.
    /// - If the remaining source offset is greater than the size of this block, that much is subtracted from the source
    ///   offset with no further action.
    /// - Otherwise, the handler is invoked with the source offset within this block, the current target offset, and the
    ///   remaining target length (`self.limit - target_offset`), and the cursor then advances accordingly.
    pub fn visit_source_block<F>(&mut self, size: u16, handler: F) -> Option<u16>
    where
        F: FnOnce(u16, u16, u16),
    {
        if let Some(target_offset) = self.target_offset {
            if self.source_offset < size {
                handler(
                    self.source_offset,
                    target_offset,
                    self.limit - target_offset,
                );

                let new_offset = target_offset + size - self.source_offset;
                self.target_offset = if new_offset < self.limit {
                    Some(new_offset)
                } else {
                    None
                };
                self.source_offset = 0;
            } else {
                self.source_offset -= size;
            }
        }
        
        self.target_offset
    }

    /// Invokes [Self::visit_source_block] if the provided context is non-empty.
    ///
    /// If the context is None, this method has no impact, and doesn't impact the source or target offsets in any way.
    pub fn visit_optional_source_block<A: ?Sized, F>(
        &mut self,
        ctx_opt: Option<&A>,
        get_size: fn(&A) -> u16,
        handler: F,
    ) -> Option<u16>
    where
        F: FnOnce(&A, u16, u16, u16),
    {
        if self.target_offset.is_some()
            && let Some(ctx) = ctx_opt
        {
            let size = get_size(ctx);
            self.visit_source_block(size, |offset, buffer_offset, limit| {
                handler(ctx, offset, buffer_offset, limit)
            })
        } else {
            self.target_offset
        }
    }
}
