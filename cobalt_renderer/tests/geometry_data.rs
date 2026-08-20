// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

use cobalt_renderer::render_tree::*;
use cobalt_renderer::resources::*;
use cobalt_renderer::{RendererErrorKind, resources::geometry::*};

mod common;

#[repr(C)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    const fn new(xyz: (f32, f32), rgb: (f32, f32, f32)) -> Vertex {
        Vertex {
            position: [xyz.0, xyz.1],
            color: [rgb.0, rgb.1, rgb.2],
        }
    }
}

const VERTEX_COUNT: usize = 7;
const VERTICES: [Vertex; VERTEX_COUNT] = [
    Vertex::new((0.0, 0.0), (1.0, 1.0, 1.0)),
    Vertex::new((1.0, 0.0), (1.0, 0.0, 0.0)),
    Vertex::new((0.5, 0.86), (1.0, 1.0, 0.0)),
    Vertex::new((-0.5, 0.86), (0.0, 1.0, 0.0)),
    Vertex::new((-1.0, 0.0), (0.0, 1.0, 1.0)),
    Vertex::new((-0.5, -0.86), (0.0, 0.0, 1.0)),
    Vertex::new((0.5, -0.86), (1.0, 0.0, 1.0)),
];

const POSITIONS: [f32; VERTEX_COUNT * 2] = [
    0.0, 0.0, 1.0, 0.0, 0.5, 0.86, -0.5, 0.86, -1.0, 0.0, -0.5, -0.86, 0.5, -0.86,
];
const POSITIONS_EXTRA: [f32; (VERTEX_COUNT + 1) * 2] = [
    0.0, 0.0, 1.0, 0.0, 0.5, 0.86, -0.5, 0.86, -1.0, 0.0, -0.5, -0.86, 0.5, -0.86, 99.0, 99.0,
];
const COLORS: [[f32; 3]; VERTEX_COUNT] = [
    [1.0, 1.0, 1.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
];
const INDEX_COUNT: usize = 18;
const INDICES: [u32; INDEX_COUNT] = [0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1];

const MIXED_INDICES: [u32; INDEX_COUNT * 2] = [
    0xffff, 0, 0xffff, 1, 0xffff, 2, 0xffff, 0, 0xffff, 2, 0xffff, 3, 0xffff, 0, 0xffff, 3, 0xffff,
    4, 0xffff, 0, 0xffff, 4, 0xffff, 5, 0xffff, 0, 0xffff, 5, 0xffff, 6, 0xffff, 0, 0xffff, 6,
    0xffff, 1,
];

const SHADER: &str = "
struct VSInput {
    float3 position : position;
    float3 color : color;
};

struct VSOutput {
    float4 position : SV_POSITION;
    float3 color : COLOR;
};

VSOutput vertex(VSInput IN)
{
    VSOutput OUT;

    OUT.color = IN.color;
    OUT.position = float4(IN.position.xy, IN.position.z, 1.0f);

    return OUT;
}

float4 fragment(VSOutput IN) : SV_TARGET0
{
    return float4(IN.color, 1.0f);
}
";

#[test]
fn geometry_data() {
    // Test different methods to layout vertex and index buffer and update their data

    let renderer = common::setup_renderer(&[]);
    let mut capture = common::setup_frame_buffer_for_capture(&renderer);

    // Setup render tree
    let color = [0.0f32, 0.0, 0.0, 0.0];
    let mut render_pass_node = renderer.create_render_pass_node();
    render_pass_node.bind_frame_buffer(&capture.frame_buffer);
    render_pass_node.set_attachment_clear_data(frame_buffers::AttachmentType::Color, 0, &color);
    renderer.set_render_passes(&[&render_pass_node], &[1]);

    let mut shader_program = renderer.create_shader_program();
    shader_program
        .load_shader_stage(
            programs::ShaderStage::Vertex,
            programs::ShaderSourceInfo::Hlsl {
                code: SHADER,
                entry_point_function_name: Some("vertex"),
            },
        )
        .unwrap();
    shader_program
        .load_shader_stage(
            programs::ShaderStage::Fragment,
            programs::ShaderSourceInfo::Hlsl {
                code: SHADER,
                entry_point_function_name: Some("fragment"),
            },
        )
        .unwrap();
    let mut shader_program = shader_program.compile_program().unwrap();

    let mut program_node = renderer.create_program_node();
    program_node
        .bind_shader_program(&mut shader_program)
        .unwrap();
    render_pass_node.add_child_node(&program_node, None);

    let mut state_group_node = renderer.create_state_group_node();
    state_group_node.set_polygon_fill_mode(PolygonFillMode::Solid);
    state_group_node.set_depth_test_enabled(false);
    program_node.add_child_node(&state_group_node);

    let mut test_vertex_layout = |succeeds: bool,
                                  f: fn(
        &mut VertexAttribute,
        &mut VertexAttribute,
        &mut UnallocatedVertexBuffer<'_>,
    )| {
        let mut positions = renderer.create_vertex_attribute(
            VertexAttributeType::Float32,
            2,
            VERTEX_COUNT,
            PerformanceHint::Default,
            PerformanceHint::Default,
            DataPersistenceFlags::PersistAlways,
        );
        let mut colors = renderer.create_vertex_attribute(
            VertexAttributeType::Float32,
            3,
            VERTEX_COUNT,
            PerformanceHint::Default,
            PerformanceHint::Default,
            DataPersistenceFlags::PersistAlways,
        );
        let mut vertex_buffer = renderer.create_vertex_buffer();

        f(&mut positions, &mut colors, &mut vertex_buffer);

        if succeeds {
            // Use buffer and display the vertex data
            vertex_buffer.allocate_memory().unwrap();

            let mut index_attribute = renderer.create_index_attribute(
                geometry::IndexAttributeType::UInt32,
                INDEX_COUNT,
                geometry::PerformanceHint::ReadNever | geometry::PerformanceHint::WriteRarely,
                geometry::PerformanceHint::ReadOften | geometry::PerformanceHint::WriteNever,
                geometry::DataPersistenceFlags::PersistAlways,
            );

            let mut index_buffer = renderer.create_index_buffer();
            index_buffer
                .bind_attribute_with_initial_data(
                    &mut index_attribute,
                    geometry::SourceData::contiguous(&INDICES),
                )
                .unwrap();
            index_buffer.allocate_memory().unwrap();

            let mut renderable = renderer.create_renderable_node();
            renderable.set_vertex_count(3, 0, 0, 0).unwrap();
            renderable
                .set_primitive_mode(PrimitiveMode::Triangles, false, false)
                .unwrap();

            renderable
                .bind_vertex_attribute(
                    &mut positions,
                    shader_program.vertex_attribute_id("position").unwrap(),
                )
                .unwrap();
            renderable
                .bind_vertex_attribute(
                    &mut colors,
                    shader_program.vertex_attribute_id("color").unwrap(),
                )
                .unwrap();
            renderable
                .bind_index_attribute(&mut index_attribute)
                .unwrap();
            renderable.set_vertex_count(INDEX_COUNT, 0, 0, 0).unwrap();

            state_group_node.add_child_node(&renderable);

            unsafe {
                renderer.start_new_frame();
            }

            static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
            common::save_capture(
                &renderer,
                &mut capture,
                &format!(
                    "geometry_{}.png",
                    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
                ),
            );

            state_group_node.remove_child_node(&renderable);
        }
    };

    // Vertex success cases

    test_vertex_layout(true, |positions, colors, buffer| {
        // Auto layout with initial data (separate)
        buffer
            .bind_attribute_with_initial_data(positions, SourceData::contiguous(&POSITIONS))
            .unwrap();
        buffer
            .bind_attribute_with_initial_data(colors, SourceData::contiguous(&COLORS))
            .unwrap();
    });

    test_vertex_layout(true, |positions, colors, buffer| {
        // Auto layout with initial data (interleaved)
        buffer
            .bind_attribute_with_initial_data(positions, SourceData::striped(&VERTICES, 0, 20, 7))
            .unwrap();
        buffer
            .bind_attribute_with_initial_data(colors, SourceData::striped(&VERTICES, 8, 20, 7))
            .unwrap();
    });

    test_vertex_layout(true, |positions, colors, buffer| {
        // Manual layout (separate) with initial data (separate)
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                positions,
                0,
                8,
                SourceData::contiguous(&POSITIONS),
            )
            .unwrap();
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                colors,
                8 * 7,
                12,
                SourceData::contiguous(&COLORS),
            )
            .unwrap();
    });

    test_vertex_layout(true, |positions, colors, buffer| {
        // Manual layout (interleaved) with initial data (separate)
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                positions,
                0,
                20,
                SourceData::contiguous(&POSITIONS),
            )
            .unwrap();
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                colors,
                8,
                20,
                SourceData::contiguous(&COLORS),
            )
            .unwrap();
    });

    test_vertex_layout(true, |positions, colors, buffer| {
        // Manual layout (separate) with initial data (interleaved)
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                positions,
                0,
                8,
                SourceData::striped(&VERTICES, 0, 20, 7),
            )
            .unwrap();
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                colors,
                8 * 7,
                12,
                SourceData::striped(&VERTICES, 8, 20, 7),
            )
            .unwrap();
    });

    test_vertex_layout(true, |positions, colors, buffer| {
        // Manual layout (interleaved) with initial data (interleaved)
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                positions,
                0,
                20,
                SourceData::striped(&VERTICES, 0, 20, 7),
            )
            .unwrap();
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                colors,
                8,
                20,
                SourceData::striped(&VERTICES, 8, 20, 7),
            )
            .unwrap();
    });

    test_vertex_layout(true, |positions, colors, buffer| {
        // Manual layout (interleaved) with raw initial data (interleaved)
        buffer
            .bind_attribute_manual_layout(positions, 0, 20)
            .unwrap();
        buffer
            .bind_attribute_manual_layout(colors, 4 * 2, 20)
            .unwrap();
        buffer.set_raw_initial_data(&VERTICES).unwrap();
    });

    test_vertex_layout(true, |positions, colors, buffer| {
        // Manual layout (interleaved) with raw initial data (interleaved)
        buffer
            .bind_attribute_manual_layout(positions, 0, 20)
            .unwrap();
        buffer
            .bind_attribute_manual_layout(colors, 4 * 2, 20)
            .unwrap();
        buffer.set_raw_initial_data(&VERTICES).unwrap();
    });

    // Vertex error cases

    test_vertex_layout(false, |positions, _, buffer| {
        // Contiguous with not enough data,
        assert!(
            buffer
                .bind_attribute_with_initial_data(
                    positions,
                    SourceData::contiguous(&POSITIONS[0..12])
                )
                .is_err()
        );
    });

    test_vertex_layout(false, |positions, _, buffer| {
        // Contiguous with too much data
        assert!(
            buffer
                .bind_attribute_with_initial_data(
                    positions,
                    SourceData::contiguous(&POSITIONS_EXTRA)
                )
                .is_err()
        );
    });

    test_vertex_layout(false, |positions, _, buffer| {
        // Contiguous with mismatched element size
        assert_eq!(
            buffer
                .bind_attribute_with_initial_data(
                    positions,
                    SourceData::contiguous(&POSITIONS[0..13])
                )
                .unwrap_err()
                .kind(),
            RendererErrorKind::BadSourceData
        );
    });

    test_vertex_layout(false, |positions, _, buffer| {
        // Offset will run off the end
        assert!(
            buffer
                .bind_attribute_with_initial_data(
                    positions,
                    SourceData::striped(&POSITIONS, 4, 8, 7)
                )
                .is_err()
        );
    });

    test_vertex_layout(false, |positions, _, buffer| {
        // Stride will run off the end
        assert!(
            buffer
                .bind_attribute_with_initial_data(
                    positions,
                    SourceData::striped(&POSITIONS, 0, 16, 7)
                )
                .is_err()
        );
    });

    test_vertex_layout(false, |positions, _, buffer| {
        // Entry count doesn't match
        assert!(
            buffer
                .bind_attribute_with_initial_data(
                    positions,
                    SourceData::striped(&POSITIONS, 0, 8, 6)
                )
                .is_err()
        );
    });

    let test_index_layout = |f: fn(&mut IndexAttribute, &mut UnallocatedIndexBuffer<'_>)| {
        let mut indices = renderer.create_index_attribute(
            IndexAttributeType::UInt32,
            INDEX_COUNT,
            PerformanceHint::Default,
            PerformanceHint::Default,
            DataPersistenceFlags::PersistAlways,
        );
        let mut index_buffer = renderer.create_index_buffer();
        f(&mut indices, &mut index_buffer);
        index_buffer.allocate_memory().unwrap();
    };

    test_index_layout(|indices, buffer| {
        // Auto layout with initial data (separate)
        buffer
            .bind_attribute_with_initial_data(indices, SourceData::contiguous(&INDICES))
            .unwrap();
    });

    test_index_layout(|indices, buffer| {
        // Auto layout with initial data (interleaved)
        buffer
            .bind_attribute_with_initial_data(
                indices,
                SourceData::striped(&MIXED_INDICES, 4, 8, INDEX_COUNT),
            )
            .unwrap();
    });

    test_index_layout(|indices, buffer| {
        // Manual layout with initial data (separate)
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                indices,
                0,
                4,
                SourceData::contiguous(&INDICES),
            )
            .unwrap();
    });

    test_index_layout(|indices, buffer| {
        // Auto layout with initial data (interleaved)
        buffer
            .bind_attribute_manual_layout_with_initial_data(
                indices,
                0,
                4,
                SourceData::striped(&MIXED_INDICES, 4, 8, INDEX_COUNT),
            )
            .unwrap();
    });
}
