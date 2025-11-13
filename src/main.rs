use raylib::prelude::*;

mod star;
mod camera;
mod uniforms;

use star::Star;
use camera::OrbitCamera;

fn main() {
    // Inicializar ventana
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Sol Procedural - Shaders + Ruido | Lab 5 Gráficas por Computadora")
        .build();

    rl.set_target_fps(60);

    // Crear cámara orbital para mejor visualización
    let mut camera = OrbitCamera::new();

    // Crear la estrella con shaders cargados
    let mut star = match Star::new(&mut rl, &thread) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error cargando shaders: {}", e);
            return;
        }
    };

    // Variables de tiempo para animación continua y cíclica
    let mut time: f32;

    println!("=== CONTROLES ===");
    println!("WASD: Rotar cámara");
    println!("Flechas Arriba/Abajo: Zoom");
    println!("SPACE: Rotación automática");
    println!("1/2: Escala de ruido");
    println!("3/4: Amplitud de ruido"); 
    println!("5/6: Amplitud de flares");
    println!("7/8: Amplitud de emisión");
    println!("9/0: Temperatura de color");
    println!("Q/E: Velocidad de pulsación");
    println!("R: Reset parámetros");
    println!("===============");

    // Bucle principal de renderizado
    while !rl.window_should_close() {
        // Actualizar tiempo para animación continua
        time = rl.get_time() as f32;

        // Procesar input de cámara
        camera.process_input(&rl);

        // Procesar input para parámetros de la estrella
        star.process_input(&rl);

        // Actualizar uniforms del shader con tiempo y parámetros
        star.update_uniforms(time);

        // === RENDERIZADO ===
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Renderizado 3D
        {
            let mut d3d = d.begin_mode3D(camera.camera);
            star.render(&mut d3d);
        }

        // UI: Mostrar información de parámetros actuales
        draw_ui(&mut d, &star);
    }
}

/// Dibujar interfaz de usuario con información de parámetros
fn draw_ui(d: &mut RaylibDrawHandle, star: &Star) {
    let params = &star.params;
    
    // Título
    d.draw_text(
        "Sol Procedural - Laboratorio 5",
        10,
        10,
        20,
        Color::YELLOW,
    );

    // Parámetros actuales
    d.draw_text(
        &format!("Noise Scale: {:.2} (1/2)", params.noise_scale),
        10,
        40,
        16,
        Color::RAYWHITE,
    );
    d.draw_text(
        &format!("Noise Amp: {:.2} (3/4)", params.noise_amp),
        10,
        60,
        16,
        Color::RAYWHITE,
    );
    d.draw_text(
        &format!("Flare Amp: {:.2} (5/6)", params.flare_amp),
        10,
        80,
        16,
        Color::RAYWHITE,
    );
    d.draw_text(
        &format!("Emission: {:.2} (7/8)", params.emission_amp),
        10,
        100,
        16,
        Color::RAYWHITE,
    );
    d.draw_text(
        &format!("Color Hotness: {:.2} (9/0)", params.color_hotness),
        10,
        120,
        16,
        Color::RAYWHITE,
    );
    d.draw_text(
        &format!("Pulse Speed: {:.2} (Q/E)", params.pulse_speed),
        10,
        140,
        16,
        Color::RAYWHITE,
    );

    // Controles
    d.draw_text("WASD: Cámara | ↑↓: Zoom | SPACE: Auto-rotar | R: Reset", 10, 680, 14, Color::LIGHTGRAY);

    // Información técnica
    d.draw_text("Ruido: Simplex + Cellular | Animación: Continua y Cíclica", 10, 700, 12, Color::GRAY);
}


