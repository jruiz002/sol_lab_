# Sol Procedural - Estrella Animada con Shaders y Ruido

Proyecto Rust + Raylib que genera una estrella/sol animada usando shaders GLSL con ruido combinado (simplex + cellular) para turbulencia, flares y emisión variable.

## Cómo Ejecutar
- `cargo run --release`
- Teclas: 1/2 (noiseScale), 3/4 (noiseAmp), 5/6 (flareAmp), 7/8 (emissionAmp), 9/0 (colorHotness)

## Animación
Video (YouTube): https://youtu.be/-EqNQghlaDk?si=6mj_RPizHt6GRTtH

## Explicación Técnica
- **Ruido Utilizado**: Combinación de Simplex noise (Ashima) para turbulencia suave y Cellular (Worley) para patrones de manchas irregulares. Simplex genera variaciones continuas, cellular añade "celdas" oscuras simulando manchas solares. Se combinan en turb/intensity con pesos (e.g., 0.5 simplex + 0.3 detail + 0.2 cellular).
- **Uniformes**:
  - `u_time`: Avanza la animación (continua/cíclica con sin(u_time * speed)).
  - `u_noiseScale`: Frecuencia del ruido (ajustable para detalle).
  - `u_noiseAmp`: Amplitud base.
  - `u_flareAmp`: Desplazamiento radial en VS para flares.
  - `u_emissionAmp`: Multiplicador de brillo en FS.
  - `u_pulseSpeed`: Velocidad de pulsación senoidal.
  - `u_colorHotness`: Controla gradiente (0: naranja, 1: azul) escalado por intensidad.
- **Efectos**: Desplazamiento en VS para forma dinámica; en FS, color e intensidad responden a ruido/tiempo para emisión variable y gradiente dinámico.

Valores default optimizados para realismo: scale=4.0, amp=0.8, flare=0.25, etc.