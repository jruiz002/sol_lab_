use raylib::prelude::*;

/// Parámetros ajustables para la estrella
#[derive(Clone, Copy, Debug)]
pub struct StarParams {
    pub noise_scale: f32,    // Escala/frecuencia del ruido 
    pub noise_amp: f32,      // Amplitud base del ruido
    pub flare_amp: f32,      // Amplitud de desplazamiento radial (flares)
    pub emission_amp: f32,   // Amplitud de emisión/luminosidad
    pub pulse_speed: f32,    // Velocidad de pulsación
    pub color_hotness: f32,  // Factor de "temperatura" del color (0=naranja, 1=azul)
}

impl Default for StarParams {
    fn default() -> Self {
        Self {
            noise_scale: 4.0,    // Frecuencia de ruido moderada para detalles visibles
            noise_amp: 0.8,      // Amplitud alta para turbulencia visible
            flare_amp: 0.25,     // Flares pronunciados para realismo solar
            emission_amp: 2.5,   // Brillo intenso característica de una estrella
            pulse_speed: 0.3,    // Pulsación lenta para efecto natural
            color_hotness: 0.5,  // Punto medio: naranja-amarillo inicial
        }
    }
}

/// Representación de la estrella con shader y parámetros
pub struct Star {
    pub shader: Shader,
    pub params: StarParams,
    
    // Ubicaciones de uniforms para optimización
    pub loc_time: i32,
    pub loc_noise_scale: i32,
    pub loc_noise_amp: i32,
    pub loc_flare_amp: i32,
    pub loc_emission_amp: i32,
    pub loc_pulse_speed: i32,
    pub loc_color_hotness: i32,
}

impl Star {
    /// Crear una nueva estrella cargando los shaders
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, String> {
        // Cargar shader (vertex + fragment)
        let shader = rl
            .load_shader(
                thread,
                Some("assets/shaders/sol.vs"),
                Some("assets/shaders/sol.fs"),
            );

        // Obtener ubicaciones de uniforms para optimización
        let loc_time = shader.get_shader_location("u_time");
        let loc_noise_scale = shader.get_shader_location("u_noiseScale");
        let loc_noise_amp = shader.get_shader_location("u_noiseAmp");
        let loc_flare_amp = shader.get_shader_location("u_flareAmp");
        let loc_emission_amp = shader.get_shader_location("u_emissionAmp");
        let loc_pulse_speed = shader.get_shader_location("u_pulseSpeed");
        let loc_color_hotness = shader.get_shader_location("u_colorHotness");

        Ok(Star {
            shader,
            params: StarParams::default(),
            loc_time,
            loc_noise_scale,
            loc_noise_amp,
            loc_flare_amp,
            loc_emission_amp,
            loc_pulse_speed,
            loc_color_hotness,
        })
    }

    /// Actualizar los uniforms del shader con los parámetros actuales
    pub fn update_uniforms(&mut self, time: f32) {
        self.shader.set_shader_value(self.loc_time, time);
        self.shader.set_shader_value(self.loc_noise_scale, self.params.noise_scale);
        self.shader.set_shader_value(self.loc_noise_amp, self.params.noise_amp);
        self.shader.set_shader_value(self.loc_flare_amp, self.params.flare_amp);
        self.shader.set_shader_value(self.loc_emission_amp, self.params.emission_amp);
        self.shader.set_shader_value(self.loc_pulse_speed, self.params.pulse_speed);
        self.shader.set_shader_value(self.loc_color_hotness, self.params.color_hotness);
    }

    /// Renderizar la estrella (esfera base con shader)
    pub fn render(&mut self, d3d: &mut RaylibMode3D<RaylibDrawHandle>) {
        let mut shader_mode = d3d.begin_shader_mode(&mut self.shader);
        // Utilizar únicamente una esfera como base (requisito técnico)
        shader_mode.draw_sphere(Vector3::new(0.0, 0.0, 0.0), 1.0, Color::WHITE);
    }

    /// Procesar input del usuario para ajustar parámetros en tiempo real
    pub fn process_input(&mut self, rl: &RaylibHandle) {
        // Controles para noise_scale (frecuencia del ruido)
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            self.params.noise_scale = (self.params.noise_scale * 1.2).min(20.0);
            println!("Noise Scale: {:.2}", self.params.noise_scale);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            self.params.noise_scale = (self.params.noise_scale * 0.8).max(0.5);
            println!("Noise Scale: {:.2}", self.params.noise_scale);
        }

        // Controles para noise_amp (amplitud del ruido)
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            self.params.noise_amp = (self.params.noise_amp * 1.1).min(3.0);
            println!("Noise Amplitude: {:.2}", self.params.noise_amp);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            self.params.noise_amp = (self.params.noise_amp * 0.9).max(0.1);
            println!("Noise Amplitude: {:.2}", self.params.noise_amp);
        }

        // Controles para flare_amp (desplazamiento radial)
        if rl.is_key_pressed(KeyboardKey::KEY_FIVE) {
            self.params.flare_amp = (self.params.flare_amp * 1.1).min(1.0);
            println!("Flare Amplitude: {:.2}", self.params.flare_amp);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SIX) {
            self.params.flare_amp = (self.params.flare_amp * 0.9).max(0.0);
            println!("Flare Amplitude: {:.2}", self.params.flare_amp);
        }

        // Controles para emission_amp (luminosidad)
        if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) {
            self.params.emission_amp = (self.params.emission_amp * 1.1).min(5.0);
            println!("Emission Amplitude: {:.2}", self.params.emission_amp);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) {
            self.params.emission_amp = (self.params.emission_amp * 0.9).max(0.5);
            println!("Emission Amplitude: {:.2}", self.params.emission_amp);
        }

        // Controles para color_hotness (temperatura de color)
        if rl.is_key_pressed(KeyboardKey::KEY_NINE) {
            self.params.color_hotness = (self.params.color_hotness + 0.05).min(1.0);
            println!("Color Hotness: {:.2}", self.params.color_hotness);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
            self.params.color_hotness = (self.params.color_hotness - 0.05).max(0.0);
            println!("Color Hotness: {:.2}", self.params.color_hotness);
        }

        // Control adicional para pulse_speed
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            self.params.pulse_speed = (self.params.pulse_speed * 1.2).min(2.0);
            println!("Pulse Speed: {:.2}", self.params.pulse_speed);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_E) {
            self.params.pulse_speed = (self.params.pulse_speed * 0.8).max(0.1);
            println!("Pulse Speed: {:.2}", self.params.pulse_speed);
        }

        // Reset a valores por defecto
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            self.params = StarParams::default();
            println!("Parámetros reseteados a valores por defecto");
        }
    }
}