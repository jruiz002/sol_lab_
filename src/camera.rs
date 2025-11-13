use raylib::prelude::*;
use std::f32::consts::PI;

/// Cámara orbital para visualizar la estrella desde diferentes ángulos
pub struct OrbitCamera {
    pub camera: Camera3D,
    pub distance: f32,
    pub angle_h: f32,  // Ángulo horizontal (yaw)
    pub angle_v: f32,  // Ángulo vertical (pitch)
    pub target: Vector3,
    pub rotation_speed: f32,
    pub zoom_speed: f32,
}

impl OrbitCamera {
    pub fn new() -> Self {
        OrbitCamera {
            camera: Camera3D::perspective(
                Vector3::new(0.0, 0.0, 4.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                45.0,
            ),
            distance: 4.0,
            angle_h: 0.0,
            angle_v: 0.0,
            target: Vector3::new(0.0, 0.0, 0.0),
            rotation_speed: 0.02,
            zoom_speed: 0.2,
        }
    }

    /// Actualizar la posición de la cámara basada en los ángulos
    fn update_position(&mut self) {
        // Limitar ángulo vertical para evitar gimbal lock
        self.angle_v = self.angle_v.clamp(-PI/2.0 + 0.1, PI/2.0 - 0.1);
        
        // Calcular nueva posición usando coordenadas esféricas
        let x = self.target.x + self.distance * self.angle_v.cos() * self.angle_h.cos();
        let y = self.target.y + self.distance * self.angle_v.sin();
        let z = self.target.z + self.distance * self.angle_v.cos() * self.angle_h.sin();
        
        self.camera.position = Vector3::new(x, y, z);
        self.camera.target = self.target;
    }

    /// Procesar input del usuario para controlar la cámara
    pub fn process_input(&mut self, rl: &RaylibHandle) {
        let mut updated = false;

        // Rotación con WASD
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.angle_h += self.rotation_speed;
            updated = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.angle_h -= self.rotation_speed;
            updated = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.angle_v += self.rotation_speed;
            updated = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.angle_v -= self.rotation_speed;
            updated = true;
        }

        // Zoom con flechas arriba/abajo
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            self.distance = (self.distance - self.zoom_speed).max(1.0);
            updated = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            self.distance = (self.distance + self.zoom_speed).min(10.0);
            updated = true;
        }

        // Rotación automática con SPACE (para demos)
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            self.angle_h += self.rotation_speed * 0.5;
            updated = true;
        }

        if updated {
            self.update_position();
        }
    }
}