use glow::*;

pub trait Geometry {
    fn draw(&self, gl: &glow::Context);
}

pub struct EmptyMesh;

impl Geometry for EmptyMesh {
    fn draw(&self, _gl: &glow::Context) {
        // do nothing, the mesh is empty
    }
}

pub mod mesh {
    use glow::{HasContext, NativeBuffer, NativeVertexArray};
    use once_cell::sync::OnceCell;

    use super::Geometry;

    static CUBE_GEOMETRY: OnceCell<(NativeBuffer, NativeVertexArray)> = OnceCell::new();

    pub struct Mesh {
        vertices: Vec<f32>,
    }

    pub fn create(vertices: Vec<f32>) -> Mesh {
        Mesh { vertices }
    }

    impl Geometry for Mesh {
        fn draw(&self, gl: &glow::Context) {
            let (_vbo, vao) = *CUBE_GEOMETRY.get_or_init(|| {
                let vertices = self.vertices.as_slice();

                let (vbo, vao) = unsafe {
                    let vertices_u8: &[u8] = core::slice::from_raw_parts(
                        vertices.as_ptr() as *const u8,
                        vertices.len() * core::mem::size_of::<f32>(),
                    );

                    let vbo = gl.create_buffer().unwrap();
                    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
                    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

                    let vao = gl.create_vertex_array().unwrap();
                    gl.bind_vertex_array(Some(vao));

                    gl.enable_vertex_attrib_array(0);
                    gl.vertex_attrib_pointer_f32(
                        0,
                        3,
                        glow::FLOAT,
                        false,
                        (5 * core::mem::size_of::<f32>()) as i32,
                        0,
                    );

                    gl.enable_vertex_attrib_array(1);
                    gl.vertex_attrib_pointer_f32(
                        1,
                        2,
                        glow::FLOAT,
                        false,
                        (5 * core::mem::size_of::<f32>()) as i32,
                        (3 * core::mem::size_of::<f32>()) as i32,
                    );

                    // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
                    // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
                    gl.bind_buffer(glow::ARRAY_BUFFER, None);
                    gl.bind_vertex_array(None);
                    (vbo, vao)
                };

                (vbo, vao)
            });

            unsafe {
                gl.bind_vertex_array(Some(vao));
                gl.draw_arrays(glow::TRIANGLES, 0, self.vertices.len() as i32 / 5);
            }
        }
    }
}

pub mod cube {
    use super::mesh::{self, Mesh};

    pub fn create() -> Mesh {
        mesh::create(vec![
            -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
            0.5, -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5,
            0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0,
            1.0, -0.5, 0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0,
            -0.5, 0.5, -0.5, 1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0,
            -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5,
            -0.5, 1.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5,
            0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0,
            1.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
            0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5,
            0.0, 1.0,
        ])
    }
}

pub mod plane {
    use glow::{HasContext, NativeBuffer, NativeVertexArray};
    use once_cell::sync::OnceCell;

    use super::Geometry;

    static CUBE_GEOMETRY: OnceCell<(NativeBuffer, NativeVertexArray)> = OnceCell::new();

    pub struct Plane {}

    pub fn create() -> Plane {
        Plane {}
    }

    impl Geometry for Plane {
        fn draw(&self, gl: &glow::Context) {
            let (_vbo, vao) = *CUBE_GEOMETRY.get_or_init(|| {
                let uv_scale = 100.0;
                let vertices: [f32; 30] = [
                    -0.5, 0.0, -0.5, 0.0, 0.0, 0.5, 0.0, -0.5, uv_scale, 0.0, 0.5, 0.0, 0.5,
                    uv_scale, uv_scale, -0.5, 0.0, -0.5, 0.0, 0.0, -0.5, 0.0, 0.5, 0.0, uv_scale,
                    0.5, 0.0, 0.5, uv_scale, uv_scale,
                ];

                let (vbo, vao) = unsafe {
                    let vertices_u8: &[u8] = core::slice::from_raw_parts(
                        vertices.as_ptr() as *const u8,
                        vertices.len() * core::mem::size_of::<f32>(),
                    );

                    let vbo = gl.create_buffer().unwrap();
                    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
                    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

                    let vao = gl.create_vertex_array().unwrap();
                    gl.bind_vertex_array(Some(vao));

                    gl.enable_vertex_attrib_array(0);
                    gl.vertex_attrib_pointer_f32(
                        0,
                        3,
                        glow::FLOAT,
                        false,
                        (5 * core::mem::size_of::<f32>()) as i32,
                        0,
                    );

                    gl.enable_vertex_attrib_array(1);
                    gl.vertex_attrib_pointer_f32(
                        1,
                        2,
                        glow::FLOAT,
                        false,
                        (5 * core::mem::size_of::<f32>()) as i32,
                        3,
                    );

                    // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
                    // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
                    gl.bind_buffer(glow::ARRAY_BUFFER, None);
                    gl.bind_vertex_array(None);
                    (vbo, vao)
                };

                (vbo, vao)
            });

            unsafe {
                gl.bind_vertex_array(Some(vao));
                gl.draw_arrays(glow::TRIANGLES, 0, 6);
            }
        }
    }
}