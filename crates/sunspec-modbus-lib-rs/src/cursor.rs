use core::cmp::min;

/// A mutable handle to support stepping through a u16 indexed target for some range of values.
///
/// Once the target has been fully traversed, further calls are treated as a no-op, short-circuiting any unnecessary
/// logic.
pub struct Cursor<E> {
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
    // Any error that may have been encountered, terminating the traversal early
    pub error: Option<E>,
}

pub enum CursorResult<E: Clone> {
    Complete,
    Incomplete(u16),
    Error(E),
}

impl<E: Clone> Cursor<E> {
    pub fn new(source_offset: u16, limit: u16) -> Self {
        Self {
            source_offset,
            target_offset: if limit > 0 { Some(0) } else { None },
            limit,
            error: None,
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.target_offset.is_none()
    }

    pub fn result(&self) -> CursorResult<E> {
        if let Some(e) = &self.error {
            CursorResult::Error(e.clone())
        } else if let Some(offset) = self.target_offset {
            CursorResult::Incomplete(offset)
        } else {
            CursorResult::Complete
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
        F: FnOnce(u16, u16, u16) -> Result<(), E>,
    {
        if let Some(target_offset) = self.target_offset {
            if self.source_offset < size {
                let limit = min(self.limit - target_offset, size - self.source_offset);

                if let Err(e) = handler(self.source_offset, target_offset, limit) {
                    self.error = Some(e);
                    self.target_offset = None;
                } else {
                    let new_offset = target_offset + limit;
                    self.target_offset = if new_offset < self.limit {
                        Some(new_offset)
                    } else {
                        None
                    };
                    self.source_offset = 0;
                }
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
        ctx_opt: Option<&mut A>,
        get_size: fn(&A) -> u16,
        handler: F,
    ) -> Option<u16>
    where
        F: FnOnce(&mut A, u16, u16, u16) -> Result<(), E>,
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

    /// Shared-reference counterpart to [Self::visit_optional_source_block], for traversals that
    /// only need read access to the context.
    pub fn visit_optional_source_block_ref<A: ?Sized, F>(
        &mut self,
        ctx_opt: Option<&A>,
        get_size: fn(&A) -> u16,
        handler: F,
    ) -> Option<u16>
    where
        F: FnOnce(&A, u16, u16, u16) -> Result<(), E>,
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

#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    use super::*;

    #[derive(Clone)]
    struct TestError;

    #[test]
    fn test_empty_cursor() {
        assert!(Cursor::<TestError>::new(5, 0).is_exhausted());
    }

    #[test]
    fn test_optional_blocks() {
        #[derive(Debug, PartialEq, Eq)]
        struct VisitArguments {
            context: Option<&'static str>,
            source_offset: u16,
            target_offset: u16,
            limit: u16,
        }

        let last_visit_args = RefCell::new(None);

        let static_visit = |source_offset, target_offset, limit| {
            *last_visit_args.borrow_mut() = Some(VisitArguments {
                context: None,
                source_offset,
                target_offset,
                limit,
            });
            Ok(())
        };

        let visit_with_context = |ctx: &mut &'static str, source_offset, target_offset, limit| {
            *last_visit_args.borrow_mut() = Some(VisitArguments {
                context: Some(ctx),
                source_offset,
                target_offset,
                limit,
            });
            Ok(())
        };

        let mut cursor: Cursor<TestError> = Cursor::new(24, 20);

        // An initial static block of 10
        assert_eq!(
            cursor.visit_source_block(10, static_visit),
            Some(0), // traverses none of the target
        );
        // Is under source offset, so isn't visited
        assert_eq!(last_visit_args.borrow_mut().take(), None);
        // But the source offset is subtracted
        assert_eq!(cursor.source_offset, 14);

        // A missing optional block
        assert_eq!(
            cursor.visit_optional_source_block(None, |_| 10, visit_with_context),
            Some(0), // traverses none of the target
        );
        // Isn't ever visited
        assert_eq!(last_visit_args.borrow_mut().take(), None);
        // And has no impact on source offset
        assert_eq!(cursor.source_offset, 14);

        // A present optional block
        assert_eq!(
            cursor.visit_optional_source_block(
                Some(&mut "initial opt block"),
                |_| 10,
                visit_with_context
            ),
            Some(0), // traverses none of the target
        );
        // Is still under the source offset, so is not visited
        assert_eq!(last_visit_args.borrow_mut().take(), None);
        // But the source offset is subtracted
        assert_eq!(cursor.source_offset, 4);

        // First visited block
        assert_eq!(
            cursor.visit_optional_source_block(
                Some(&mut "second opt block"),
                |_| 10,
                visit_with_context
            ),
            Some(6), // traverses 6 after skipping the 4 offset
        );
        // Is visited - with the remaining source offset passed in to the handler
        assert_eq!(
            last_visit_args.borrow_mut().take(),
            Some(VisitArguments {
                context: Some("second opt block"),
                source_offset: 4,
                target_offset: 0,
                limit: 6
            })
        );

        // And the remaining source offset is subtracted
        assert_eq!(cursor.source_offset, 0);

        // Another missing optional block
        assert_eq!(
            cursor.visit_optional_source_block(None, |_| 10, visit_with_context),
            Some(6), // traverses none of the target
        );
        // Isn't ever visited
        assert_eq!(last_visit_args.borrow_mut().take(), None);
        // And has no impact on source offset
        assert_eq!(cursor.source_offset, 0);

        // Middle visited block
        assert_eq!(
            cursor.visit_optional_source_block(
                Some(&mut "third opt block"),
                |_| 10,
                visit_with_context
            ),
            Some(16), // traverses the next 10 offset
        );
        // Is visited without internal offset, and provided the full slice of the target from 6-16
        assert_eq!(
            last_visit_args.borrow_mut().take(),
            Some(VisitArguments {
                context: Some("third opt block"),
                source_offset: 0,
                target_offset: 6,
                limit: 10
            })
        );

        // Final visited block
        assert_eq!(
            cursor.visit_optional_source_block(
                Some(&mut "fourth opt block"),
                |_| 10,
                visit_with_context
            ),
            None, // traverses the remaining offset - this None can be used as a signal to return early from the caller
        );
        // Is visited without internal offset, but target slice only includes the final 4 registers
        assert_eq!(
            last_visit_args.borrow_mut().take(),
            Some(VisitArguments {
                context: Some("fourth opt block"),
                source_offset: 0,
                target_offset: 16,
                limit: 4
            })
        );

        // Any subsequent block won't be visited
        assert_eq!(
            cursor.visit_optional_source_block(
                Some(&mut "fifth opt block"),
                |_| 10,
                visit_with_context
            ),
            None,
        );
        assert_eq!(last_visit_args.borrow_mut().take(), None);
    }
}
