# Sol Procedural - Laboratorio 5 Gráficas por Computadora

Una estrella generada proceduralmente usando shaders GLSL y funciones de ruido, implementada en Rust con Raylib.

## 🌟 Características

### Cumplimiento de Requisitos Técnicos
- ✅ **Esfera base única**: Solo se utiliza una esfera como geometría base
- ✅ **Sin texturas/materiales**: Todo el aspecto visual se genera mediante shaders
- ✅ **Animación continua**: Usando `uniform float time` para animación cíclica
- ✅ **Funciones de ruido**: Implementación de Simplex y Cellular noise
- ✅ **Parámetros ajustables**: Controles en tiempo real para todos los parámetros

### Funcionalidades Implementadas
- 🔥 **Emisión variable**: Simulación de luminosidad y picos de energía
- 🌪️ **Distorsión visual**: Desplazamiento radial en Vertex Shader para flares
- 🎨 **Gradiente dinámico**: Control de color basado en intensidad/temperatura
- 🎛️ **Parámetros ajustables**: Control en tiempo real de todos los aspectos

## 🎮 Controles

### Cámara
- `WASD`: Rotar cámara orbital
- `↑↓`: Zoom in/out
- `SPACE`: Rotación automática

### Parámetros de la Estrella
- `1/2`: Aumentar/Disminuir escala de ruido (frecuencia)
- `3/4`: Aumentar/Disminuir amplitud de ruido (intensidad turbulencia)
- `5/6`: Aumentar/Disminuir amplitud de flares (desplazamiento radial)
- `7/8`: Aumentar/Disminuir emisión (luminosidad)
- `9/0`: Aumentar/Disminuir temperatura de color (naranja → azul)
- `Q/E`: Aumentar/Disminuir velocidad de pulsación
- `R`: Reset a valores por defecto

## 🛠️ Cómo Ejecutar

```bash
cargo run --release
```

## 🎬 Demostración

![Animación de la Estrella](demo.gif)

Video demostración: [YouTube Link](https://youtu.be/oZPAKLF-CW0?si=UXVf3DDdWRwABUyb)

## 🧮 Implementación Técnica

### Shaders

#### Vertex Shader (`sol.vs`)
- **Desplazamiento radial**: Modifica posición de vértices usando ruido
- **Múltiples octavas**: Combina Simplex y Cellular noise
- **Animación temporal**: Usa `u_time` para movimiento continuo
- **Pulsación**: Efecto de respiración de la estrella

#### Fragment Shader (`sol.fs`)
- **Gradiente de color**: Naranja → Amarillo → Blanco → Azul
- **Emisión variable**: Simula picos de energía
- **Combinación de ruidos**: Mezcla Simplex y Cellular para detalles
- **Control de temperatura**: Parámetro `u_colorHotness` para color dinámico

### Uniforms del Shader

| Uniform | Tipo | Descripción | Rango |
|---------|------|-------------|-------|
| `u_time` | `float` | Tiempo para animación continua | 0.0+ |
| `u_noiseScale` | `float` | Frecuencia del ruido (detalle) | 0.5-20.0 |
| `u_noiseAmp` | `float` | Amplitud del ruido (intensidad) | 0.1-3.0 |
| `u_flareAmp` | `float` | Desplazamiento radial máximo | 0.0-1.0 |
| `u_emissionAmp` | `float` | Multiplicador de emisión/brillo | 0.5-5.0 |
| `u_pulseSpeed` | `float` | Velocidad de pulsación | 0.1-2.0 |
| `u_colorHotness` | `float` | Factor de temperatura (0=frío, 1=caliente) | 0.0-1.0 |

### Efectos Visuales

#### Emisión Variable
- **Implementación**: Modulación de brillo con ruido y pulsación
- **Efecto**: Simula picos de energía y actividad solar variable
- **Fórmula**: `emission = baseColor * (0.7 + 0.3 * pulse) * u_emissionAmp * (1.0 + intensity)`

#### Distorsión/Flare
- **Implementación**: Desplazamiento de vértices en dirección normal
- **Efecto**: Crea prominencias y flares característicos del sol
- **Fórmula**: `position += normal * flareAmplitude * (noise + pulse)`

#### Control de Color por Temperatura
```glsl
vec3 colorGradient(float t) {
    vec3 c1 = vec3(1.0, 0.45, 0.0);   // naranja
    vec3 c2 = vec3(1.0, 0.9, 0.4);    // amarillo  
    vec3 c3 = vec3(1.0, 1.0, 0.9);    // blanco cálido
    vec3 c4 = vec3(0.8, 0.9, 1.0);    // azul pálido (muy caliente)
    // ... interpolación suave entre colores
}
```

**Gradiente físico**: Basado en temperatura de cuerpo negro, donde la intensidad del ruido modula la temperatura local.

## 📁 Estructura del Proyecto

```
sol_procedural/
├── src/
│   ├── main.rs          # Loop principal y UI
│   ├── star.rs          # Lógica de la estrella y shaders
│   ├── camera.rs        # Sistema de cámara orbital
│   ├── uniforms.rs      # Manejo de uniforms
│   └── lib.rs           # Módulos
├── assets/
│   └── shaders/
│       ├── sol.vs       # Vertex shader
│       └── sol.fs       # Fragment shader
└── Cargo.toml
```

## 🔬 Aspectos Técnicos Destacados

### Optimizaciones
- **Shaders eficientes**: Implementaciones estándar optimizadas
- **Modularización**: Código separado por responsabilidades
- **UI informativa**: Feedback inmediato sin sobrecargar rendimiento

### Realismo Físico
- **Pulsación natural**: Frecuencia baja para realismo
- **Gradiente basado en física**: Temperaturas de cuerpo negro
- **Manchas solares**: Cellular noise para patrones convincentes
- **Flares dinámicos**: Desplazamiento radial realista

---

*Desarrollado para el curso de Gráficas por Computadora - Universidad del Valle de Guatemala*  