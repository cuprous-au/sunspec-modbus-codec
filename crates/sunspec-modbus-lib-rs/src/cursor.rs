pub struct Cursor {
    pub source_offset: u16,
    pub buffer_offset: u16,
    pub limit: u16,
}

impl Cursor {
    pub fn handle_static_read<F>(&mut self, size: u16, handler: F) -> Option<()>
    where
        F: FnOnce(u16, u16, u16),
    {
        if self.source_offset < size {
            handler(
                self.source_offset,
                self.buffer_offset,
                self.limit - self.buffer_offset,
            );
            self.buffer_offset += size - self.source_offset;
            self.source_offset = 0;
        } else {
            self.source_offset -= size;
        }

        if self.buffer_offset < self.limit {
            Some(())
        } else {
            None
        }
    }

    pub fn handle_adapter_read<A: ?Sized, F>(
        &mut self,
        adapter_opt: Option<&A>,
        get_size: fn(&A) -> u16,
        handler: F,
    ) -> Option<()>
    where
        F: FnOnce(&A, u16, u16, u16),
    {
        if let Some(adapter) = adapter_opt {
            let size = get_size(adapter);
            self.handle_static_read(size, |offset, buffer_offset, limit| {
                handler(adapter, offset, buffer_offset, limit)
            })
        } else {
            Some(())
        }
    }
}
