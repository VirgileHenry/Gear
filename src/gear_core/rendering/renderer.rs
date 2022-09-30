use std::{collections::HashMap};
use cgmath::SquareMatrix;
use foundry::iterate_over_component;
use crate::gear_core::{
    rendering::{
        mesh::MeshRenderer,
        camera::CameraComponent,
    },
    geometry::transform::Transform,
};

use super::{shaders::{ShaderProgram, Shader}, mesh::{Mesh, MeshType, self}};


/// R is the renderer itself
pub trait Renderer {
    fn render(&self, components: &mut foundry::ecs::component_table::ComponentTable);
}

pub struct DefaultOpenGlRenderer {
    shader_programs: HashMap<u32, ShaderProgram>,
    missing_shader_program: ShaderProgram,
}

impl DefaultOpenGlRenderer {
    pub fn new() -> DefaultOpenGlRenderer {
        use crate::gear_core::rendering::shaders_files::shaders::{MISSING_FRAG_SHADER, DEFAULT_VERT_SHADER};
        DefaultOpenGlRenderer {
            shader_programs: HashMap::new(),
            missing_shader_program: ShaderProgram::simple_program(MISSING_FRAG_SHADER, DEFAULT_VERT_SHADER)
                .expect("[GEAR ENGINE] -> [RENDERER] -> Unable to compile default shaders : "),
        }
    }

    pub fn register_shader_program(&mut self, program: ShaderProgram) {
        self.shader_programs.insert(program.id() as u32, program);
    }
}

impl Renderer for DefaultOpenGlRenderer {
    fn render(&self, components: &mut foundry::ecs::component_table::ComponentTable) {
        // found main camera

        for (camera, cam_transform) in iterate_over_component!(&components; CameraComponent, Transform) {
            if !camera.is_main() { continue; } // check we have the main camera
            // sort elements to render by shader to minimise the change of shader program
            let mut rendering_map: HashMap<u32, Vec<(&Transform, &MeshRenderer)>> = HashMap::new();

            for (transform, mesh) in iterate_over_component!(&components; Transform, MeshRenderer) {
                match rendering_map.get_mut(&mesh.material.program_ref.id()) {
                    Some(vec) => vec.push((transform, mesh)),
                    None => {rendering_map.insert(mesh.material.program_ref.id(), vec![(transform, mesh)]);},
                }
            }

            for (id, vec) in rendering_map.into_iter() {
                // switch to render program
                let current_program = match self.shader_programs.get(&id) {
                    Some(shader_program) => shader_program,
                    None => &self.missing_shader_program,
                };
                current_program.set_used();
                // set camera uniform
                current_program.set_mat4("cameraWorldPos", cam_transform.world_pos());
                current_program.set_mat4("projectionMat", camera.view_matrix());

                for (transform, mesh_renderer) in vec.into_iter() {
                    // todo !
                    // set model uniform
                    current_program.set_mat4("modelWorldPos", transform.world_pos());
                    // bind the vertex array
                    mesh_renderer.vao().bind();
                    // (bind textures)
                    // (change states)
                    // draw elements (glDrawArrays or glDrawElements)
                    unsafe {
                        gl::DrawElements(
                            gl::TRIANGLES, // mode
                            mesh_renderer.triangles_len() as i32,             // starting index in the enabled arrays
                            gl::UNSIGNED_INT,
                            0 as *const std::ffi::c_void, // number of indices to be rendered
                        );
                    }
                }
            }

            break; // render only once in case there are multiple main camera component (and avoid useless shooting)
        }
    }
}
