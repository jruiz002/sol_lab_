/// Estructura para manejar uniformes del shader de manera organizada
pub struct ShaderUniforms {
    pub time: f32,
    pub noise_scale: f32,
    pub noise_amp: f32,
    pub flare_amp: f32,
    pub emission_amp: f32,
    pub pulse_speed: f32,
    pub color_hotness: f32,
}

impl Default for ShaderUniforms {
    fn default() -> Self {
        Self {
            time: 0.0,
            noise_scale: 4.0,
            noise_amp: 0.8,
            flare_amp: 0.25,
            emission_amp: 2.5,
            pulse_speed: 0.3,
            color_hotness: 0.5,
        }
    }
}

impl ShaderUniforms {
    /// Crear nuevos uniformes con valores por defecto
    pub fn new() -> Self {
        Self::default()
    }

    /// Actualizar el tiempo (llamar cada frame)
    pub fn update_time(&mut self, time: f32) {
        self.time = time;
    }

    /// Validar que los valores estén en rangos apropiados
    pub fn validate(&mut self) {
        self.noise_scale = self.noise_scale.clamp(0.1, 50.0);
        self.noise_amp = self.noise_amp.clamp(0.0, 5.0);
        self.flare_amp = self.flare_amp.clamp(0.0, 2.0);
        self.emission_amp = self.emission_amp.clamp(0.1, 10.0);
        self.pulse_speed = self.pulse_speed.clamp(0.05, 5.0);
        self.color_hotness = self.color_hotness.clamp(0.0, 1.0);
    }
}