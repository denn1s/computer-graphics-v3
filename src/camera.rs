use raylib::prelude::*;

pub struct Camera {
    pub eye: Vector3,
    pub center: Vector3,
    pub up: Vector3,
    pub forward: Vector3,
    pub right: Vector3,
    pub view_matrix: Matrix,
    pub has_changed: bool,
}

impl Camera {
    pub fn new(eye: Vector3, center: Vector3, up: Vector3) -> Self {
        let mut camera = Camera {
            eye,
            center,
            up,
            forward: Vector3::zero(),
            right: Vector3::zero(),
            view_matrix: Matrix::identity(),
            has_changed: true,
        };
        camera.update_view_matrix();
        camera
    }

    pub fn update_view_matrix(&mut self) {
        self.forward = (self.center - self.eye).normalized();
        self.right = self.forward.cross(self.up).normalized();
        self.up = self.right.cross(self.forward); // Recalculate up vector

        self.view_matrix = Matrix::look_at(self.eye, self.center, self.up);
    }

    pub fn orbit(&mut self, horizontal_angle: f32, vertical_angle: f32) {
        let rotation_horizontal = Matrix::rotate(self.up, horizontal_angle);
        let rotation_vertical = Matrix::rotate(self.right, vertical_angle);

        let new_eye = self.eye - self.center;
        let mut new_eye_vec4 = Vector4::new(new_eye.x, new_eye.y, new_eye.z, 1.0);

        new_eye_vec4 = new_eye_vec4.transform(rotation_horizontal);
        new_eye_vec4 = new_eye_vec4.transform(rotation_vertical);

        self.eye = self.center + Vector3::new(new_eye_vec4.x, new_eye_vec4.y, new_eye_vec4.z);

        self.update_view_matrix();
        self.has_changed = true;
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalized();
        self.eye += direction * delta;
        self.update_view_matrix();
        self.has_changed = true;
    }

    pub fn basis_change(&self, v: &Vector3) -> Vector3 {
        let inverted_view = self.view_matrix.inverted();
        let v_homogeneous = Vector4::new(v.x, v.y, v.z, 0.0);
        let rotated_v = v_homogeneous.transform(inverted_view);
        Vector3::new(rotated_v.x, rotated_v.y, rotated_v.z)
    }

    pub fn is_changed(&mut self) -> bool {
        if self.has_changed {
            self.has_changed = false;
            return true;
        }
        false
    }
}
