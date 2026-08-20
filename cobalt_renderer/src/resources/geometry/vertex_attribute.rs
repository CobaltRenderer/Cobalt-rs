// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use std::sync::Arc;

use super::{DataPersistenceFlags, PerformanceHint};
use crate::RendererResult;
use crate::renderer::{Renderer, RendererInternal};
use crate::resources::batching::TransferBatch;

use cobalt_renderer_sys as sys;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexAttributeType {
    Int8 = sys::Cobalt_VertexAttributeType_Int8 as u32,
    Int16 = sys::Cobalt_VertexAttributeType_Int16 as u32,
    Int32 = sys::Cobalt_VertexAttributeType_Int32 as u32,
    UInt8 = sys::Cobalt_VertexAttributeType_UInt8 as u32,
    UInt16 = sys::Cobalt_VertexAttributeType_UInt16 as u32,
    UInt32 = sys::Cobalt_VertexAttributeType_UInt32 as u32,
    Norm8 = sys::Cobalt_VertexAttributeType_Norm8 as u32,
    Norm16 = sys::Cobalt_VertexAttributeType_Norm16 as u32,
    UNorm8 = sys::Cobalt_VertexAttributeType_UNorm8 as u32,
    UNorm16 = sys::Cobalt_VertexAttributeType_UNorm16 as u32,
    Float16 = sys::Cobalt_VertexAttributeType_Float16 as u32,
    Float32 = sys::Cobalt_VertexAttributeType_Float32 as u32,
    A2B10G10R10UNorm = sys::Cobalt_VertexAttributeType_A2B10G10R10UNorm as u32,
}

impl From<sys::Cobalt_VertexAttributeType> for VertexAttributeType {
    fn from(value: sys::Cobalt_VertexAttributeType) -> Self {
        match value {
            sys::Cobalt_VertexAttributeType_Int8 => Self::Int8,
            sys::Cobalt_VertexAttributeType_Int16 => Self::Int16,
            sys::Cobalt_VertexAttributeType_Int32 => Self::Int32,
            sys::Cobalt_VertexAttributeType_UInt8 => Self::UInt8,
            sys::Cobalt_VertexAttributeType_UInt16 => Self::UInt16,
            sys::Cobalt_VertexAttributeType_UInt32 => Self::UInt32,
            sys::Cobalt_VertexAttributeType_Norm8 => Self::Norm8,
            sys::Cobalt_VertexAttributeType_Norm16 => Self::Norm16,
            sys::Cobalt_VertexAttributeType_UNorm8 => Self::UNorm8,
            sys::Cobalt_VertexAttributeType_UNorm16 => Self::UNorm16,
            sys::Cobalt_VertexAttributeType_Float16 => Self::Float16,
            sys::Cobalt_VertexAttributeType_Float32 => Self::Float32,
            sys::Cobalt_VertexAttributeType_A2B10G10R10UNorm => Self::A2B10G10R10UNorm,
            v => panic!("Encountered unknown VertexAttributeType value '{v}'"),
        }
    }
}

pub struct VertexAttribute {
    pub(crate) handle: sys::Cobalt_VertexAttribute,
    _renderer: Arc<RendererInternal>,
    pub(crate) element_size: usize,
}

impl VertexAttribute {
    pub(crate) fn new(
        renderer: &Renderer,
        data_type: VertexAttributeType,
        element_count: usize,
        vertex_count: usize,
        performance_hint_cpu: PerformanceHint,
        performance_hint_gpu: PerformanceHint,
        data_persistence_flags: DataPersistenceFlags,
    ) -> VertexAttribute {
        assert!(
            element_count > 0 && element_count <= 4,
            "VertexAttribute must have element count from 1 to 4, cannot be {element_count}"
        );

        let data_size = match data_type {
            VertexAttributeType::Int8 => 1,
            VertexAttributeType::Int16 => 2,
            VertexAttributeType::Int32 => 4,
            VertexAttributeType::UInt8 => 1,
            VertexAttributeType::UInt16 => 2,
            VertexAttributeType::UInt32 => 4,
            VertexAttributeType::Norm8 => 1,
            VertexAttributeType::Norm16 => 2,
            VertexAttributeType::UNorm8 => 1,
            VertexAttributeType::UNorm16 => 2,
            VertexAttributeType::Float16 => 2,
            VertexAttributeType::Float32 => 4,
            VertexAttributeType::A2B10G10R10UNorm => 4,
        };

        let mut vertex_attribute = std::ptr::null_mut();
        unsafe {
            sys::Cobalt_Renderer_CreateVertexAttribute(
                renderer.handle(),
                &mut vertex_attribute,
                data_type as sys::Cobalt_VertexAttributeType,
                element_count,
                vertex_count,
                performance_hint_cpu.bits() as sys::Cobalt_VertexPerformanceHint,
                performance_hint_gpu.bits() as sys::Cobalt_VertexPerformanceHint,
                data_persistence_flags.bits() as sys::Cobalt_VertexDataPersistenceFlags,
            );
        }

        VertexAttribute {
            _renderer: renderer.internal_clone(),
            handle: vertex_attribute,
            element_size: data_size * element_count,
        }
    }

    pub fn is_bound_to_buffer(&self) -> bool {
        unsafe { sys::Cobalt_VertexAttribute_IsBoundToBuffer(self.handle) != 0 }
    }

    pub fn vertex_count(&self) -> usize {
        unsafe { sys::Cobalt_VertexAttribute_GetVertexCount(self.handle) }
    }

    pub fn element_count(&self) -> usize {
        unsafe { sys::Cobalt_VertexAttribute_GetAttributeElementCount(self.handle) }
    }

    pub fn data_type(&self) -> VertexAttributeType {
        let value = unsafe { sys::Cobalt_VertexAttribute_GetDataType(self.handle) };
        VertexAttributeType::from(value)
    }

    pub fn data_persistence_flags(&self) -> DataPersistenceFlags {
        unsafe {
            DataPersistenceFlags::from_bits_truncate(
                sys::Cobalt_VertexAttribute_GetDataPersistenceFlags(self.handle) as u32,
            )
        }
    }

    pub fn performance_hint_cpu(&self) -> PerformanceHint {
        unsafe {
            PerformanceHint::from_bits_truncate(sys::Cobalt_VertexAttribute_GetPerformanceHintCpu(
                self.handle,
            ) as u32)
        }
    }

    pub fn performance_hint_gpu(&self) -> PerformanceHint {
        unsafe {
            PerformanceHint::from_bits_truncate(sys::Cobalt_VertexAttribute_GetPerformanceHintGpu(
                self.handle,
            ) as u32)
        }
    }

    pub fn queue_data_update<S: Sized>(
        &mut self,
        source_data: super::SourceData<'_, S>,
        initial_vertex_no: usize,
        transfer_batch: Option<&TransferBatch>,
    ) -> RendererResult<()> {
        source_data.validate(self.element_size)?;
        let transfer_batch = match transfer_batch {
            None => std::ptr::null_mut(),
            Some(b) => b.handle,
        };
        unsafe {
            return_on_failure!(sys::Cobalt_VertexAttribute_QueueDataUpdate(
                self.handle,
                source_data.buffer(),
                source_data.entry_count(self.element_size),
                initial_vertex_no,
                source_data.stride(self.element_size),
                transfer_batch,
            ))
        }
        Ok(())
    }
}

impl Drop for VertexAttribute {
    fn drop(&mut self) {
        unsafe { sys::Cobalt_VertexAttribute_Delete(self.handle) }
    }
}

unsafe impl Send for VertexAttribute {}
unsafe impl Sync for VertexAttribute {}
