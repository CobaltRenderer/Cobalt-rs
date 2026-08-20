// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use bitflags::bitflags;

mod index_attribute;
mod index_buffer;
mod vertex_attribute;
mod vertex_buffer;

pub use index_attribute::*;
pub use index_buffer::*;
pub use vertex_attribute::*;
pub use vertex_buffer::*;

use crate::{RendererError, RendererErrorKind, RendererResult};

// NOTE(DTM): Cobalt technically defines different performance hints
// and persistence flags for vertex and index attributes/buffers
// However, they are identical in practice and unlikely to change
// Therefore, we have a common set here for both, but this could
// change in the future if the two are not identical

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PerformanceHint : u32 {
        const Default = 0x00000000;
        const ReadNever = 0x00000001;
        const ReadRarely = 0x00000002;
        const ReadOften = 0x00000004;
        const ReadFlagsMask = 0x000000FF;
        const WriteNever = 0x00000100;
        const WriteRarely = 0x00000200;
        const WriteOften = 0x00000400;
        const WriteFlagsMask = 0x0000FF00;
    }
}

impl Default for PerformanceHint {
    fn default() -> Self {
        PerformanceHint::Default
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DataPersistenceFlags : u32 {
        const PersistAlways = 0x00000000;
        const InvalidateExistingDataOnWrite = 0x000000001;
        const InvalidateExistingDataAfterDrawComplete = 0x000000002;
    }
}

impl Default for DataPersistenceFlags {
    fn default() -> Self {
        Self::PersistAlways
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceData<'a, S: Sized> {
    Contiguous {
        buffer: &'a [S],
    },
    Striped {
        buffer: &'a [S],
        start_offset_in_bytes: usize,
        stride_in_bytes: usize,
        entry_count: usize,
    },
}

impl<'a, S: Sized> SourceData<'a, S> {
    pub fn contiguous(buffer: &'a [S]) -> Self {
        SourceData::Contiguous { buffer }
    }

    pub fn striped(
        buffer: &'a [S],
        start_offset_in_bytes: usize,
        stride_in_bytes: usize,
        entry_count: usize,
    ) -> Self {
        SourceData::Striped {
            buffer,
            start_offset_in_bytes,
            stride_in_bytes,
            entry_count,
        }
    }

    pub fn validate(&self, element_size: usize) -> RendererResult<()> {
        match self {
            Self::Contiguous { buffer } => {
                let byte_count = core::mem::size_of_val(*buffer);
                if byte_count % element_size != 0 {
                    return Err(RendererError::new_with_error(
                        RendererErrorKind::BadSourceData,
                        format!("SourceData buffer is {} bytes in size, which is not divisible by attribute element size of {}", byte_count, element_size).into()
                    ));
                }
                Ok(())
            }
            Self::Striped {
                buffer,
                start_offset_in_bytes,
                stride_in_bytes,
                entry_count,
            } => {
                let byte_count = core::mem::size_of_val(*buffer);
                if *stride_in_bytes == 0 {
                    return Err(RendererError::new_with_error(
                        RendererErrorKind::BadSourceData,
                        "stride_in_bytes cannot be 0".into(),
                    ));
                }
                if *entry_count == 0 {
                    return Err(RendererError::new_with_error(
                        RendererErrorKind::BadSourceData,
                        "entry_count cannot be 0".into(),
                    ));
                }
                let end_byte = start_offset_in_bytes
                    .saturating_add(element_size)
                    .saturating_add(stride_in_bytes.saturating_mul(*entry_count - 1));
                if end_byte > byte_count {
                    return Err(RendererError::new_with_error(
                        RendererErrorKind::BadSourceData,
                        "entry_count cannot be 0".into(),
                    ));
                }
                Ok(())
            }
        }
    }

    pub(crate) unsafe fn buffer(&self) -> *const u8 {
        match self {
            Self::Contiguous { buffer } => buffer.as_ptr() as *const u8,
            Self::Striped {
                buffer,
                start_offset_in_bytes,
                stride_in_bytes: _,
                entry_count: _,
            } => unsafe { (buffer.as_ptr() as *const u8).add(*start_offset_in_bytes) },
        }
    }

    pub(crate) fn stride(&self, element_size: usize) -> usize {
        match self {
            Self::Contiguous { buffer: _ } => element_size,
            Self::Striped {
                buffer: _,
                start_offset_in_bytes: _,
                stride_in_bytes,
                entry_count: _,
            } => *stride_in_bytes,
        }
    }

    pub(crate) fn entry_count(&self, element_size: usize) -> usize {
        match self {
            Self::Contiguous { buffer } => core::mem::size_of_val(*buffer) / element_size,
            Self::Striped {
                buffer: _,
                start_offset_in_bytes: _,
                stride_in_bytes: _,
                entry_count,
            } => *entry_count,
        }
    }
}
