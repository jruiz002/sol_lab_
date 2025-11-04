use raylib::prelude::*;

fn main() {
    // Ventana
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Sol Procedural - Shaders + Ruido")
        .build();

    rl.set_target_fps(60);

    // Cámara simple
    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 0.0, 4.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    // Cargar shader (vertex + fragment)
    let mut shader = rl
        .load_shader(
            &thread,
            Some("assets/shaders/sol.vs"),
            Some("assets/shaders/sol.fs"),
        );

    // Obtener ubicaciones de uniforms (API nueva en raylib-rs: sobre Shader)
    let loc_time = shader.get_shader_location("u_time");
    let loc_noise_scale = shader.get_shader_location("u_noiseScale");
    let loc_noise_amp = shader.get_shader_location("u_noiseAmp");
    let loc_flare_amp = shader.get_shader_location("u_flareAmp");
    let loc_emission_amp = shader.get_shader_location("u_emissionAmp");
    let loc_pulse_speed = shader.get_shader_location("u_pulseSpeed");
    let loc_color_hotness = shader.get_shader_location("u_colorHotness");

    // Parámetros ajustables
    let mut time: f32 = 0.0;
    let mut params = StarParams::default();

    // Valores iniciales de uniforms
    // Los uniforms se setean dentro del bloque de dibujo (API de raylib-rs)

    // Bucle principal
    while !rl.window_should_close() {
        // Tiempo animación
        time = rl.get_time() as f32; // continuo y cíclico con sin/cos en el shader

        // Controles simples para ajustar parámetros
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            params.noise_scale *= 1.2;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            params.noise_scale *= 0.8;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            params.noise_amp *= 1.1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            params.noise_amp *= 0.9;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FIVE) {
            params.flare_amp *= 1.1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SIX) {
            params.flare_amp *= 0.9;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) {
            params.emission_amp *= 1.1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) {
            params.emission_amp *= 0.9;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_NINE) {
            params.color_hotness = (params.color_hotness + 0.05).min(1.0);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
            params.color_hotness = (params.color_hotness - 0.05).max(0.0);
        }

        // Dibujado
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Set de uniforms durante el frame ANTES de entrar en shader mode
        shader.set_shader_value(loc_time, time);
        shader.set_shader_value(loc_noise_scale, params.noise_scale);
        shader.set_shader_value(loc_noise_amp, params.noise_amp);
        shader.set_shader_value(loc_flare_amp, params.flare_amp);
        shader.set_shader_value(loc_emission_amp, params.emission_amp);
        shader.set_shader_value(loc_pulse_speed, params.pulse_speed);
        shader.set_shader_value(loc_color_hotness, params.color_hotness);

        // Modo 3D + Shader RAII
        {
            let mut d3d = d.begin_mode3D(camera);
            {
                let mut sm = d3d.begin_shader_mode(&mut shader);
                // Esfera base obligatoria
                sm.draw_sphere(Vector3::new(0.0, 0.0, 0.0), 1.0, Color::WHITE);
            } // end shader mode
        } // end mode3D

        d.draw_text(
            &format!(
                "1/2 scale {:.2} | 3/4 amp {:.2} | 5/6 flare {:.2} | 7/8 emission {:.2} | 9/0 hot {:.2}",
                params.noise_scale, params.noise_amp, params.flare_amp, params.emission_amp, params.color_hotness
            ),
            10,
            10,
            18,
            Color::RAYWHITE,
        );
    }
}

#[derive(Clone, Copy)]
struct StarParams {
    noise_scale: f32,
    noise_amp: f32,
    flare_amp: f32,
    emission_amp: f32,
    pulse_speed: f32,
    color_hotness: f32,
}

impl Default for StarParams {
    fn default() -> Self {
        Self {
            noise_scale: 4.0, // Aumentado para más detalle
            noise_amp: 0.8,   // Mayor amplitud para turbulencia visible
            flare_amp: 0.25,  // Flares más pronunciados para realismo solar
            emission_amp: 2.5, // Brillo más intenso
            pulse_speed: 0.3, // Pulsación más lenta para efecto natural
            color_hotness: 0.5, // Medio para naranja-amarillo inicial
        }
    }
}
