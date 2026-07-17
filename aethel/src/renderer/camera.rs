use std::ffi::CString;

use gl;
use glfw::PWindow;
use nalgebra_glm as glm;

use crate::shader::Shader;

pub struct Camera {
    pub position: glm::Vec3,
    pub orientation: glm::Vec3,
    pub up: glm::Vec3,

    pub width: f32,
    pub height: f32,

    pub speed: f32,
    pub sensitivity: f32,

    first_click: bool,
}

impl Camera {
    pub fn new(width: f32, height: f32, position: glm::Vec3) -> Self {
        Self {
            position,

            orientation: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),

            width,
            height,

            speed: 5.0,
            sensitivity: 100.0,

            first_click: true,
        }
    }

    pub fn matrix(
        &self,
        fov_deg: f32,
        near_plane: f32,
        far_plane: f32,
        shader: &Shader,
        uniform: &str,
    ) {
        let view = glm::look_at(
            &self.position,
            &(self.position + self.orientation),
            &self.up,
        );

        let projection = glm::perspective(
            self.width / self.height,
            fov_deg.to_radians(),
            near_plane,
            far_plane,
        );

        let camera_matrix = projection * view;

        let uniform = CString::new(uniform).unwrap();

        unsafe {
            let location = gl::GetUniformLocation(
                shader.id,
                uniform.as_ptr(),
            );

            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                camera_matrix.as_ptr(),
            );
        }
    }

    pub fn handle_inputs(
        &mut self,
        window: &mut PWindow,
        delta_time: f32,
    ) {
        use glfw::{Action, Key, MouseButton, CursorMode};

        // Release mouse
        if window.get_key(Key::Escape) == Action::Press {
            window.set_cursor_mode(CursorMode::Normal);
            self.first_click = true;
        }


        // Capture mouse
        if window.get_mouse_button(MouseButton::Button1)
            == Action::Press
        {
            window.set_cursor_mode(CursorMode::Disabled);

            if self.first_click {
                window.set_cursor_pos(
                    (self.width / 2.0) as f64,
                    (self.height / 2.0) as f64,
                );

                self.first_click = false;
            }
        }


        let forward = glm::normalize(&self.orientation);

        let right = glm::normalize(
            &glm::cross(&forward, &self.up)
        );


        // Sprint
        if window.get_key(Key::LeftShift)
            == Action::Press
        {
            self.speed = 10.0;
        }
        else {
            self.speed = 5.0;
        }


        // Movement

        if window.get_key(Key::W)
            == Action::Press
        {
            self.position +=
                forward * self.speed * delta_time;
        }


        if window.get_key(Key::S)
            == Action::Press
        {
            self.position -=
                forward * self.speed * delta_time;
        }


        if window.get_key(Key::A)
            == Action::Press
        {
            self.position -=
                right * self.speed * delta_time;
        }


        if window.get_key(Key::D)
            == Action::Press
        {
            self.position +=
                right * self.speed * delta_time;
        }


        // Fly up/down

        if window.get_key(Key::Space)
            == Action::Press
        {
            self.position +=
                self.up * self.speed * delta_time;
        }


        if window.get_key(Key::LeftControl)
            == Action::Press
        {
            self.position -=
                self.up * self.speed * delta_time;
        }



        // Mouse rotation

        if window.get_cursor_mode()
            == CursorMode::Disabled
        {
            let (mouse_x, mouse_y) =
                window.get_cursor_pos();


            let rot_x =
                self.sensitivity *
                ((mouse_y as f32 - self.height / 2.0)
                    / self.height);


            let rot_y =
                self.sensitivity *
                ((mouse_x as f32 - self.width / 2.0)
                    / self.width);



            // Pitch

            let axis =
                glm::normalize(
                    &glm::cross(
                        &self.orientation,
                        &self.up
                    )
                );


            let rotation =
                glm::rotate_vec3(
                    &self.orientation,
                    (-rot_x).to_radians(),
                    &axis,
                );


            let angle =
                glm::angle(
                    &rotation,
                    &self.up
                );


            // Prevent flipping camera

            if (angle - 90f32.to_radians()).abs()
                <= 85f32.to_radians()
            {
                self.orientation = rotation;
            }



            // Yaw

            self.orientation =
                glm::rotate_vec3(
                    &self.orientation,
                    (-rot_y).to_radians(),
                    &self.up,
                );



            // Reset mouse center

            window.set_cursor_pos(
                (self.width / 2.0) as f64,
                (self.height / 2.0) as f64,
            );
        }
    }
}