// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License
use bitflags::bitflags;

use crate::{render_tree::StateContainer, resources::TextureId};

use cobalt_renderer_sys as sys;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    R = sys::Cobalt_ImageFormat_R as u32,
    RG = sys::Cobalt_ImageFormat_RG as u32,
    RGB = sys::Cobalt_ImageFormat_RGB as u32,
    RGBA = sys::Cobalt_ImageFormat_RGBA as u32,
    BGR = sys::Cobalt_ImageFormat_BGR as u32,
    BGRA = sys::Cobalt_ImageFormat_BGRA as u32,
    X = sys::Cobalt_ImageFormat_X as u32,
    XY = sys::Cobalt_ImageFormat_XY as u32,
    XYZ = sys::Cobalt_ImageFormat_XYZ as u32,
    XYZW = sys::Cobalt_ImageFormat_XYZW as u32,
    Depth = sys::Cobalt_ImageFormat_Depth as u32,
    DepthAndStencil = sys::Cobalt_ImageFormat_DepthAndStencil as u32,
}

impl From<sys::Cobalt_ImageFormat> for ImageFormat {
    fn from(value: sys::Cobalt_ImageFormat) -> Self {
        match value {
            sys::Cobalt_ImageFormat_R => Self::R,
            sys::Cobalt_ImageFormat_RG => Self::RG,
            sys::Cobalt_ImageFormat_RGB => Self::RGB,
            sys::Cobalt_ImageFormat_RGBA => Self::RGBA,
            sys::Cobalt_ImageFormat_BGR => Self::BGR,
            sys::Cobalt_ImageFormat_BGRA => Self::BGRA,
            sys::Cobalt_ImageFormat_X => Self::X,
            sys::Cobalt_ImageFormat_XY => Self::XY,
            sys::Cobalt_ImageFormat_XYZ => Self::XYZ,
            sys::Cobalt_ImageFormat_XYZW => Self::XYZW,
            sys::Cobalt_ImageFormat_Depth => Self::Depth,
            sys::Cobalt_ImageFormat_DepthAndStencil => Self::DepthAndStencil,
            v => panic!("Encountered unknown ImageFormat value '{v}'"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceImageFormat {
    R = sys::Cobalt_SourceImageFormat_R as u32,
    RG = sys::Cobalt_SourceImageFormat_RG as u32,
    RGB = sys::Cobalt_SourceImageFormat_RGB as u32,
    RGBA = sys::Cobalt_SourceImageFormat_RGBA as u32,
    BGR = sys::Cobalt_SourceImageFormat_BGR as u32,
    BGRA = sys::Cobalt_SourceImageFormat_BGRA as u32,
    X = sys::Cobalt_SourceImageFormat_X as u32,
    XY = sys::Cobalt_SourceImageFormat_XY as u32,
    XYZ = sys::Cobalt_SourceImageFormat_XYZ as u32,
    XYZW = sys::Cobalt_SourceImageFormat_XYZW as u32,
}

impl From<sys::Cobalt_SourceImageFormat> for SourceImageFormat {
    fn from(value: sys::Cobalt_SourceImageFormat) -> Self {
        match value {
            sys::Cobalt_SourceImageFormat_R => Self::R,
            sys::Cobalt_SourceImageFormat_RG => Self::RG,
            sys::Cobalt_SourceImageFormat_RGB => Self::RGB,
            sys::Cobalt_SourceImageFormat_RGBA => Self::RGBA,
            sys::Cobalt_SourceImageFormat_BGR => Self::BGR,
            sys::Cobalt_SourceImageFormat_BGRA => Self::BGRA,
            sys::Cobalt_SourceImageFormat_X => Self::X,
            sys::Cobalt_SourceImageFormat_XY => Self::XY,
            sys::Cobalt_SourceImageFormat_XYZ => Self::XYZ,
            sys::Cobalt_SourceImageFormat_XYZW => Self::XYZW,
            v => panic!("Encountered unknown SourceImageFormat value '{v}'"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Int8 = sys::Cobalt_DataFormat_Int8 as u32,
    Int16 = sys::Cobalt_DataFormat_Int16 as u32,
    Int32 = sys::Cobalt_DataFormat_Int32 as u32,
    UInt8 = sys::Cobalt_DataFormat_UInt8 as u32,
    UInt16 = sys::Cobalt_DataFormat_UInt16 as u32,
    UInt32 = sys::Cobalt_DataFormat_UInt32 as u32,
    Norm8 = sys::Cobalt_DataFormat_Norm8 as u32,
    Norm16 = sys::Cobalt_DataFormat_Norm16 as u32,
    UNorm8 = sys::Cobalt_DataFormat_UNorm8 as u32,
    UNorm16 = sys::Cobalt_DataFormat_UNorm16 as u32,
    Float16 = sys::Cobalt_DataFormat_Float16 as u32,
    Float32 = sys::Cobalt_DataFormat_Float32 as u32,
    DXT1 = sys::Cobalt_DataFormat_DXT1 as u32,
    DXT3 = sys::Cobalt_DataFormat_DXT3 as u32,
    DXT5 = sys::Cobalt_DataFormat_DXT5 as u32,
    ETC2 = sys::Cobalt_DataFormat_ETC2 as u32,
    BPTC = sys::Cobalt_DataFormat_BPTC as u32,
    ASTC4x4 = sys::Cobalt_DataFormat_ASTC4x4 as u32,
    ASTC5x5 = sys::Cobalt_DataFormat_ASTC5x5 as u32,
    ASTC6x6 = sys::Cobalt_DataFormat_ASTC6x6 as u32,
    ASTC8x8 = sys::Cobalt_DataFormat_ASTC8x8 as u32,
    DepthUNorm16 = sys::Cobalt_DataFormat_DepthUNorm16 as u32,
    DepthUNorm24 = sys::Cobalt_DataFormat_DepthUNorm24 as u32,
    DepthUNorm24StencilUInt8 = sys::Cobalt_DataFormat_DepthUNorm24StencilUInt8 as u32,
    DepthFloat32 = sys::Cobalt_DataFormat_DepthFloat32 as u32,
    DepthFloat32StencilUInt8 = sys::Cobalt_DataFormat_DepthFloat32StencilUInt8 as u32,
}

impl From<sys::Cobalt_DataFormat> for DataFormat {
    fn from(value: sys::Cobalt_DataFormat) -> Self {
        match value {
            sys::Cobalt_DataFormat_Int8 => Self::Int8,
            sys::Cobalt_DataFormat_Int16 => Self::Int16,
            sys::Cobalt_DataFormat_Int32 => Self::Int32,
            sys::Cobalt_DataFormat_UInt8 => Self::UInt8,
            sys::Cobalt_DataFormat_UInt16 => Self::UInt16,
            sys::Cobalt_DataFormat_UInt32 => Self::UInt32,
            sys::Cobalt_DataFormat_Norm8 => Self::Norm8,
            sys::Cobalt_DataFormat_Norm16 => Self::Norm16,
            sys::Cobalt_DataFormat_UNorm8 => Self::UNorm8,
            sys::Cobalt_DataFormat_UNorm16 => Self::UNorm16,
            sys::Cobalt_DataFormat_Float16 => Self::Float16,
            sys::Cobalt_DataFormat_Float32 => Self::Float32,
            sys::Cobalt_DataFormat_DXT1 => Self::DXT1,
            sys::Cobalt_DataFormat_DXT3 => Self::DXT3,
            sys::Cobalt_DataFormat_DXT5 => Self::DXT5,
            sys::Cobalt_DataFormat_ETC2 => Self::ETC2,
            sys::Cobalt_DataFormat_BPTC => Self::BPTC,
            sys::Cobalt_DataFormat_ASTC4x4 => Self::ASTC4x4,
            sys::Cobalt_DataFormat_ASTC5x5 => Self::ASTC5x5,
            sys::Cobalt_DataFormat_ASTC6x6 => Self::ASTC6x6,
            sys::Cobalt_DataFormat_ASTC8x8 => Self::ASTC8x8,
            sys::Cobalt_DataFormat_DepthUNorm16 => Self::DepthUNorm16,
            sys::Cobalt_DataFormat_DepthUNorm24 => Self::DepthUNorm24,
            sys::Cobalt_DataFormat_DepthUNorm24StencilUInt8 => Self::DepthUNorm24StencilUInt8,
            sys::Cobalt_DataFormat_DepthFloat32 => Self::DepthFloat32,
            sys::Cobalt_DataFormat_DepthFloat32StencilUInt8 => Self::DepthFloat32StencilUInt8,
            v => panic!("Encountered unknown DataFormat value '{v}'"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceDataFormat {
    Int8 = sys::Cobalt_SourceDataFormat_Int8 as u32,
    Int16 = sys::Cobalt_SourceDataFormat_Int16 as u32,
    Int32 = sys::Cobalt_SourceDataFormat_Int32 as u32,
    UInt8 = sys::Cobalt_SourceDataFormat_UInt8 as u32,
    UInt16 = sys::Cobalt_SourceDataFormat_UInt16 as u32,
    UInt32 = sys::Cobalt_SourceDataFormat_UInt32 as u32,
    Norm8 = sys::Cobalt_SourceDataFormat_Norm8 as u32,
    Norm16 = sys::Cobalt_SourceDataFormat_Norm16 as u32,
    Norm32 = sys::Cobalt_SourceDataFormat_Norm32 as u32,
    UNorm8 = sys::Cobalt_SourceDataFormat_UNorm8 as u32,
    UNorm16 = sys::Cobalt_SourceDataFormat_UNorm16 as u32,
    UNorm32 = sys::Cobalt_SourceDataFormat_UNorm32 as u32,
    Float16 = sys::Cobalt_SourceDataFormat_Float16 as u32,
    Float32 = sys::Cobalt_SourceDataFormat_Float32 as u32,
    DXT1 = sys::Cobalt_SourceDataFormat_DXT1 as u32,
    DXT3 = sys::Cobalt_SourceDataFormat_DXT3 as u32,
    DXT5 = sys::Cobalt_SourceDataFormat_DXT5 as u32,
    ETC2 = sys::Cobalt_SourceDataFormat_ETC2 as u32,
    BPTC = sys::Cobalt_SourceDataFormat_BPTC as u32,
    ASTC4x4 = sys::Cobalt_SourceDataFormat_ASTC4x4 as u32,
    ASTC5x5 = sys::Cobalt_SourceDataFormat_ASTC5x5 as u32,
    ASTC6x6 = sys::Cobalt_SourceDataFormat_ASTC6x6 as u32,
    ASTC8x8 = sys::Cobalt_SourceDataFormat_ASTC8x8 as u32,
}

impl From<sys::Cobalt_SourceDataFormat> for SourceDataFormat {
    fn from(value: sys::Cobalt_SourceDataFormat) -> Self {
        match value {
            sys::Cobalt_SourceDataFormat_Int8 => Self::Int8,
            sys::Cobalt_SourceDataFormat_Int16 => Self::Int16,
            sys::Cobalt_SourceDataFormat_Int32 => Self::Int32,
            sys::Cobalt_SourceDataFormat_UInt8 => Self::UInt8,
            sys::Cobalt_SourceDataFormat_UInt16 => Self::UInt16,
            sys::Cobalt_SourceDataFormat_UInt32 => Self::UInt32,
            sys::Cobalt_SourceDataFormat_Norm8 => Self::Norm8,
            sys::Cobalt_SourceDataFormat_Norm16 => Self::Norm16,
            sys::Cobalt_SourceDataFormat_Norm32 => Self::Norm32,
            sys::Cobalt_SourceDataFormat_UNorm8 => Self::UNorm8,
            sys::Cobalt_SourceDataFormat_UNorm16 => Self::UNorm16,
            sys::Cobalt_SourceDataFormat_UNorm32 => Self::UNorm32,
            sys::Cobalt_SourceDataFormat_Float16 => Self::Float16,
            sys::Cobalt_SourceDataFormat_Float32 => Self::Float32,
            sys::Cobalt_SourceDataFormat_DXT1 => Self::DXT1,
            sys::Cobalt_SourceDataFormat_DXT3 => Self::DXT3,
            sys::Cobalt_SourceDataFormat_DXT5 => Self::DXT5,
            sys::Cobalt_SourceDataFormat_ETC2 => Self::ETC2,
            sys::Cobalt_SourceDataFormat_BPTC => Self::BPTC,
            sys::Cobalt_SourceDataFormat_ASTC4x4 => Self::ASTC4x4,
            sys::Cobalt_SourceDataFormat_ASTC5x5 => Self::ASTC5x5,
            sys::Cobalt_SourceDataFormat_ASTC6x6 => Self::ASTC6x6,
            sys::Cobalt_SourceDataFormat_ASTC8x8 => Self::ASTC8x8,
            v => panic!("Encountered unknown SourceDataFormat value '{v}'"),
        }
    }
}

pub const fn equivalent_image_formats(
    source_format: SourceImageFormat,
    format: ImageFormat,
) -> bool {
    match source_format {
        SourceImageFormat::R | SourceImageFormat::X => {
            matches!(format, ImageFormat::R | ImageFormat::X)
        }
        SourceImageFormat::RG | SourceImageFormat::XY => {
            matches!(format, ImageFormat::RG | ImageFormat::XY)
        }
        SourceImageFormat::RGB | SourceImageFormat::XYZ => {
            matches!(format, ImageFormat::RGB | ImageFormat::XYZ)
        }
        SourceImageFormat::RGBA | SourceImageFormat::XYZW => {
            matches!(format, ImageFormat::RGBA | ImageFormat::XYZW)
        }
        SourceImageFormat::BGR => matches!(format, ImageFormat::BGR),
        SourceImageFormat::BGRA => matches!(format, ImageFormat::BGRA),
    }
}

impl SourceImageFormat {
    pub const fn element_count_per_pixel(&self) -> usize {
        match self {
            Self::R | Self::X => 1,
            Self::RG | Self::XY => 2,
            Self::RGB | Self::BGR | Self::XYZ => 3,
            Self::RGBA | Self::BGRA | Self::XYZW => 4,
        }
    }
}

impl ImageFormat {
    pub const fn element_count_per_pixel(&self) -> usize {
        match self {
            Self::R | Self::X | Self::Depth | Self::DepthAndStencil => 1,
            Self::RG | Self::XY => 2,
            Self::RGB | Self::BGR | Self::XYZ => 3,
            Self::RGBA | Self::BGRA | Self::XYZW => 4,
        }
    }
}

pub const fn equivalent_data_formats(source_format: SourceDataFormat, format: DataFormat) -> bool {
    match source_format {
        SourceDataFormat::Int8 => matches!(format, DataFormat::Int8),
        SourceDataFormat::Int16 => matches!(format, DataFormat::Int16),
        SourceDataFormat::Int32 => matches!(format, DataFormat::Int32),
        SourceDataFormat::UNorm8 | SourceDataFormat::UInt8 => {
            matches!(format, DataFormat::UInt8 | DataFormat::UNorm8)
        }
        SourceDataFormat::UNorm16 | SourceDataFormat::UInt16 => matches!(
            format,
            DataFormat::UInt16 | DataFormat::UNorm16 | DataFormat::DepthUNorm16
        ),
        SourceDataFormat::UInt32 => matches!(format, DataFormat::UInt32),
        SourceDataFormat::Norm8 => matches!(format, DataFormat::Norm8),
        SourceDataFormat::Norm16 => matches!(format, DataFormat::Norm16),
        SourceDataFormat::Float16 => matches!(format, DataFormat::Float16),
        SourceDataFormat::Float32 => {
            matches!(format, DataFormat::Float32 | DataFormat::DepthFloat32)
        }
        SourceDataFormat::DXT1 => matches!(format, DataFormat::DXT1),
        SourceDataFormat::DXT3 => matches!(format, DataFormat::DXT3),
        SourceDataFormat::DXT5 => matches!(format, DataFormat::DXT5),
        SourceDataFormat::ETC2 => matches!(format, DataFormat::ETC2),
        SourceDataFormat::BPTC => matches!(format, DataFormat::BPTC),
        SourceDataFormat::ASTC4x4 => matches!(format, DataFormat::ASTC4x4),
        SourceDataFormat::ASTC5x5 => matches!(format, DataFormat::ASTC5x5),
        SourceDataFormat::ASTC6x6 => matches!(format, DataFormat::ASTC6x6),
        SourceDataFormat::ASTC8x8 => matches!(format, DataFormat::ASTC8x8),
        _ => false,
    }
}

pub const fn cell_size_in_bytes_for_format(
    image_format: ImageFormat,
    data_format: DataFormat,
) -> usize {
    match data_format {
        DataFormat::DXT1 => 8,
        DataFormat::DXT3
        | DataFormat::DXT5
        | DataFormat::BPTC
        | DataFormat::ETC2
        | DataFormat::ASTC4x4
        | DataFormat::ASTC5x5
        | DataFormat::ASTC6x6
        | DataFormat::ASTC8x8 => 16,
        f => image_format.element_count_per_pixel() * f.bytes_per_element(),
    }
}

pub const fn cell_size_in_bytes_for_source_format(
    image_format: SourceImageFormat,
    data_format: SourceDataFormat,
) -> usize {
    match data_format {
        SourceDataFormat::DXT1 => 8,
        SourceDataFormat::DXT3
        | SourceDataFormat::DXT5
        | SourceDataFormat::BPTC
        | SourceDataFormat::ETC2
        | SourceDataFormat::ASTC4x4
        | SourceDataFormat::ASTC5x5
        | SourceDataFormat::ASTC6x6
        | SourceDataFormat::ASTC8x8 => 16,
        f => image_format.element_count_per_pixel() * f.bytes_per_element(),
    }
}

impl DataFormat {
    pub const fn bytes_per_element(&self) -> usize {
        match self {
            Self::Int8 | Self::UInt8 | Self::Norm8 | Self::UNorm8 => 1,
            Self::Int16 | Self::UInt16 | Self::Norm16 | Self::UNorm16 | Self::Float16 => 2,
            Self::Int32 | Self::UInt32 | Self::Float32 => 4,
            Self::DepthUNorm16 => 2,
            Self::DepthUNorm24 | Self::DepthUNorm24StencilUInt8 | Self::DepthFloat32 => 4,
            Self::DepthFloat32StencilUInt8 => 8,
            _ => 0,
        }
    }

    pub const fn is_compressed_format(&self) -> bool {
        matches!(
            self,
            Self::DXT1
                | Self::DXT3
                | Self::DXT5
                | Self::ETC2
                | Self::BPTC
                | Self::ASTC4x4
                | Self::ASTC5x5
                | Self::ASTC6x6
                | Self::ASTC8x8
        )
    }

    pub const fn cell_dimensions_in_pixels(&self) -> [u32; 2] {
        match self {
            Self::DXT1 | Self::DXT3 | Self::DXT5 | Self::ETC2 | Self::BPTC | Self::ASTC4x4 => {
                [4, 4]
            }
            Self::ASTC5x5 => [5, 5],
            Self::ASTC6x6 => [6, 6],
            Self::ASTC8x8 => [8, 8],
            _ => [1, 1],
        }
    }
}

impl SourceDataFormat {
    pub const fn bytes_per_element(&self) -> usize {
        match self {
            Self::Int8 | Self::UInt8 | Self::Norm8 | Self::UNorm8 => 1,
            Self::Int16 | Self::UInt16 | Self::Norm16 | Self::UNorm16 | Self::Float16 => 2,
            Self::Int32 | Self::UInt32 | Self::Norm32 | Self::UNorm32 | Self::Float32 => 4,
            _ => 0,
        }
    }

    pub const fn is_compressed_format(&self) -> bool {
        matches!(
            self,
            Self::DXT1
                | Self::DXT3
                | Self::DXT5
                | Self::ETC2
                | Self::BPTC
                | Self::ASTC4x4
                | Self::ASTC5x5
                | Self::ASTC6x6
                | Self::ASTC8x8
        )
    }

    pub const fn cell_dimensions_in_pixels(&self) -> [u32; 2] {
        match self {
            Self::DXT1 | Self::DXT3 | Self::DXT5 | Self::ETC2 | Self::BPTC | Self::ASTC4x4 => {
                [4, 4]
            }
            Self::ASTC5x5 => [5, 5],
            Self::ASTC6x6 => [6, 6],
            Self::ASTC8x8 => [8, 8],
            _ => [1, 1],
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeMapFace {
    PositiveX = sys::Cobalt_CubeMapFace_PositiveX as u32,
    NegativeX = sys::Cobalt_CubeMapFace_NegativeX as u32,
    PositiveY = sys::Cobalt_CubeMapFace_PositiveY as u32,
    NegativeY = sys::Cobalt_CubeMapFace_NegativeY as u32,
    PositiveZ = sys::Cobalt_CubeMapFace_PositiveZ as u32,
    NegativeZ = sys::Cobalt_CubeMapFace_NegativeZ as u32,
}

impl From<sys::Cobalt_CubeMapFace> for CubeMapFace {
    fn from(value: sys::Cobalt_CubeMapFace) -> Self {
        match value {
            sys::Cobalt_CubeMapFace_PositiveX => Self::PositiveX,
            sys::Cobalt_CubeMapFace_NegativeX => Self::NegativeX,
            sys::Cobalt_CubeMapFace_PositiveY => Self::PositiveY,
            sys::Cobalt_CubeMapFace_NegativeY => Self::NegativeY,
            sys::Cobalt_CubeMapFace_PositiveZ => Self::PositiveZ,
            sys::Cobalt_CubeMapFace_NegativeZ => Self::NegativeZ,
            v => panic!("Encountered unknown CubeMapFace value '{v}'"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleCount {
    SampleCount1 = sys::Cobalt_SampleCount_1 as u32,
    SampleCount2 = sys::Cobalt_SampleCount_2 as u32,
    SampleCount4 = sys::Cobalt_SampleCount_4 as u32,
    SampleCount8 = sys::Cobalt_SampleCount_8 as u32,
    SampleCount16 = sys::Cobalt_SampleCount_16 as u32,
    SampleCount32 = sys::Cobalt_SampleCount_32 as u32,
}

impl From<sys::Cobalt_SampleCount> for SampleCount {
    fn from(value: sys::Cobalt_SampleCount) -> Self {
        match value {
            sys::Cobalt_SampleCount_1 => Self::SampleCount1,
            sys::Cobalt_SampleCount_2 => Self::SampleCount2,
            sys::Cobalt_SampleCount_4 => Self::SampleCount4,
            sys::Cobalt_SampleCount_8 => Self::SampleCount8,
            sys::Cobalt_SampleCount_16 => Self::SampleCount16,
            sys::Cobalt_SampleCount_32 => Self::SampleCount32,
            v => panic!("Encountered unknown SampleCount value '{v}'"),
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TextureUsageFlags : u32
    {
        const Default = sys::Cobalt_TextureUsageFlags_Default as u32;
        const ShaderInput = sys::Cobalt_TextureUsageFlags_ShaderInput as u32;
        const FrameBufferOutput = sys::Cobalt_TextureUsageFlags_FrameBufferOutput as u32;
        const MultiSampleResolve = sys::Cobalt_TextureUsageFlags_MultiSampleResolve as u32;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PerformanceHint : u32
    {
        const Default = sys::Cobalt_TexturePerformanceHint_Default as u32;
        const ReadNever = sys::Cobalt_TexturePerformanceHint_ReadNever as u32;
        const ReadRarely = sys::Cobalt_TexturePerformanceHint_ReadRarely as u32;
        const ReadOften = sys::Cobalt_TexturePerformanceHint_ReadOften as u32;
        const ReadFlagsMask = sys::Cobalt_TexturePerformanceHint_ReadFlagsMask as u32;
        const WriteNever = sys::Cobalt_TexturePerformanceHint_WriteNever as u32;
        const WriteRarely = sys::Cobalt_TexturePerformanceHint_WriteRarely as u32;
        const WriteOften = sys::Cobalt_TexturePerformanceHint_WriteOften as u32;
        const WriteFlagsMask = sys::Cobalt_TexturePerformanceHint_WriteFlagsMask as u32;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DataPersistenceFlags : u32 {
        const PersistAlways = sys::Cobalt_TextureDataPersistenceFlags_PersistAlways as u32;
        const InvalidateExistingDataOnWrite = sys::Cobalt_TextureDataPersistenceFlags_InvalidateExistingDataOnWrite as u32;
        const InvalidateExistingDataAfterDrawComplete = sys::Cobalt_TextureDataPersistenceFlags_InvalidateExistingDataAfterDrawComplete as u32;
    }
}

// This is a workaround for Rust not having generic specialization
// which would allow us to have different functions under the same name
// that would take different input types and have different implementations.
// Instead we have a trait which we only implement on some types
// which then have specializations. Then we can have one generic
// function which takes this trait and calls the specialized function
// on the type. Not ideal but functional

pub trait TextureBuffer {
    #[doc(hidden)]
    fn texture_handle(&self) -> sys::Cobalt_TextureBuffer;

    #[doc(hidden)]
    fn bind_to_state_container(
        &mut self,
        texture_id: TextureId,
        container: &mut impl StateContainer,
    );
}
