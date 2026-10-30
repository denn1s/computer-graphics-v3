use super::*;

#[test]
fn combiner_interpolates_mask_endpoints() {
    let hot = Vector3::new(1.0, 0.4, 0.1);
    let cold = Vector3::new(0.0, 0.1, 0.5);
    let black = shader_examples::console_styles::combine(hot, cold, Vector3::zero(), cold);
    let white =
        shader_examples::console_styles::combine(hot, cold, Vector3::new(1.0, 1.0, 1.0), cold);
    assert!((black - cold).length() < 0.00001);
    assert!((white - hot).length() < 0.00001);
}

#[test]
fn surface_coordinates_are_perspective_correct() {
    let uniforms = Uniforms::new(8, 8);
    let make_vertex = |screen_position, inverse_w, object_position| vertex::RasterVertex {
        screen_position,
        inverse_w,
        object_position,
        world_position: object_position,
        normal: Vector3::new(0.0, 0.0, 1.0),
        uv: Vector2::zero(),
        intensity: 1.0,
    };
    let vertices = [
        make_vertex(Vector3::new(0.0, 0.0, 0.0), 1.0, Vector3::zero()),
        make_vertex(
            Vector3::new(4.0, 0.0, 0.0),
            0.5,
            Vector3::new(1.0, 0.0, 0.0),
        ),
        make_vertex(
            Vector3::new(0.0, 4.0, 0.0),
            0.25,
            Vector3::new(0.0, 1.0, 0.0),
        ),
    ];
    let fragments = triangle::triangle(&vertices, &uniforms);
    let fragment = fragments
        .iter()
        .find(|f| f.position == Vector2::new(0.5, 0.5))
        .unwrap();
    let reciprocal = 0.75 + 0.125 * 0.5 + 0.125 * 0.25;
    assert!((fragment.object_position.x - 0.125 * 0.5 / reciprocal).abs() < 0.00001);
    assert!((fragment.object_position.y - 0.125 * 0.25 / reciprocal).abs() < 0.00001);
}

#[test]
fn all_presets_render_finite_colors_and_keep_same_silhouette() {
    let vertices = obj::load("assets/models/sphere.obj").unwrap();
    let mut uniforms = Uniforms::new(80, 60);
    uniforms.time = 2.0;
    let mut framebuffer = Framebuffer::new(80, 60);
    let mut first_depths = Vec::new();
    for index in 0..shaders::NAMES.len() {
        uniforms.shader_index = index;
        render(&mut framebuffer, &vertices, &uniforms);
        assert!(framebuffer.depths.iter().any(|d| d.is_finite()));
        assert!(
            framebuffer
                .colors
                .iter()
                .all(|c| c.x.is_finite() && c.y.is_finite() && c.z.is_finite())
        );
        if index == 0 {
            first_depths = framebuffer.depths.clone();
        } else {
            assert_eq!(first_depths, framebuffer.depths);
        }
    }
}

#[test]
fn bloom_spreads_light_to_neighboring_pixels() {
    let mut scene = vec![Vector3::zero(); 25 * 25];
    scene[12 * 25 + 12] = Vector3::new(1.0, 1.0, 1.0);
    let result = shader_examples::passes::bloom(&scene, 25, 25);
    assert!(result[12 * 25 + 13].x > 0.0);
    assert_eq!(result[0], Vector3::zero());
}

#[test]
fn sphere_stays_centered_at_both_window_sizes() {
    let vertices = obj::load("assets/models/sphere.obj").unwrap();
    for (width, height) in [(800, 600), (WINDOW_WIDTH, WINDOW_HEIGHT)] {
        let uniforms = Uniforms::new(width, height);
        let mut framebuffer = Framebuffer::new(width, height);
        render(&mut framebuffer, &vertices, &uniforms);
        let mut min_x = width;
        let mut max_x = 0;
        let mut min_y = height;
        let mut max_y = 0;
        for (index, depth) in framebuffer.depths.iter().enumerate() {
            if depth.is_finite() {
                min_x = min_x.min(index % width);
                max_x = max_x.max(index % width);
                min_y = min_y.min(index / width);
                max_y = max_y.max(index / width);
            }
        }
        let center_x = (min_x + max_x + 1) as f32 / 2.0;
        let center_y = (min_y + max_y + 1) as f32 / 2.0;
        assert!((center_x - width as f32 / 2.0).abs() <= 1.0);
        assert!((center_y - height as f32 / 2.0).abs() <= 1.0);
    }
}

#[test]
fn bloom_handles_a_non_square_framebuffer() {
    let (width, height) = (37, 19);
    let mut scene = vec![Vector3::zero(); width * height];
    scene[(height - 2) * width + width - 2] = Vector3::new(1.0, 1.0, 1.0);
    let result = shader_examples::passes::bloom(&scene, width, height);
    assert_eq!(result.len(), width * height);
    assert!(result[(height - 1) * width + width - 1].x > 0.0);
    assert_eq!(result[0], Vector3::zero());
}
