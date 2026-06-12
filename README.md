# Cádiz 1812

Juego narrativo-político ambientado en la Cádiz de 1812, durante las Cortes de Cádiz. Explora los eventos históricos que marcaron la redacción de la primera constitución española.

## Requisitos

- **Sistema Operativo**: Debian, Ubuntu o cualquier distribución Linux compatible
- **Rust**: Versión 1.70 o superior (recomendado: última versión estable)
- **Servidor Gráfico**: Wayland o X11 configurado (para interfaz Slint)
- **Herramientas de desarrollo**: git, cargo, build-essential

### Dependencias opcionales (para Slint)
```bash
# Para Slint con backend GTK (opcional)
sudo apt install libgtk-4-dev
```

## Instalación

1. Clonar el repositorio:
```bash
git clone https://github.com/alvrui/cadiz12.git
cd cadiz12
```

2. Compilar el proyecto:
```bash
cargo build --release
```

3. Para desarrollo:
```bash
cargo build
```

## Ejecución

### Ejecutar el juego principal
```bash
cargo run --release
```

### Ejecutar ejemplos

#### Simulación completa
```bash
cargo run --example simular
```

#### Juego terminal
```bash
cargo run --example terminal_game configs/basic/completa.json
```

#### Generar configuración
```bash
cargo run --example generar_config
```

### Verificar todos los ejemplos
```bash
cargo check --examples
```

## Arquitectura

El proyecto está estructurado en **3 niveles principales**:

### 1. Nivel de Configuración (`src/config/`)
- **mod.rs**: Módulo principal de configuración
- **partida.rs**: Estructuras de configuración de partida
- **medidores.rs**: Definición de medidores del juego (popularidad, presupuesto, etc.)

Contiene todas las estructuras de datos que definen el estado inicial del juego.

### 2. Nivel de Motor (`src/engine/`)
- **mod.rs**: API principal del motor de juego
- **m1_tiempo.rs**: Gestión del tiempo (jornadas, actos, tramos)
- **m2_protagonista.rs**: Estado y acciones del protagonista
- **m3_medidores.rs**: Lógica de cálculo y actualización de medidores
- **m4_generador_eventos.rs**: Generación dinámica de eventos basados en el estado
- **m5_eventos.rs**: Definición y resolución de eventos
- **m6_memoria.rs**: Sistema de memoria histórica del juego
- **dtos.rs**: Data Transfer Objects para comunicación entre módulos

El motor implementa la lógica de juego, gestión de estado y resolución de eventos.

### 3. Nivel de Interfaz (`src/ui/`)
- **mod.rs**: Módulo principal de UI
- **slint.rs**: Interfaz gráfica usando Slint 1.3

Proporciona la interfaz de usuario para interactuar con el motor del juego.

### Módulo SDK (`src/sdk/`)
- **generador.rs**: Herramientas de generación de contenido
- **cliente_mistral.rs**: Integración con API de Mistral para generación de contenido

## Configuración

### Estructura del archivo JSON de partida

```json
{
  "titulo": "Nombre de la partida",
  "descripcion": "Descripción de la partida",
  "protagonista": {
    "nombre": "Nombre del protagonista",
    "descripcion": "Descripción del personaje",
    "medidores": [
      {
        "nombre": "Popularidad",
        "valor": 50,
        "tendencia": 0,
        "umbral_bajo": 20,
        "umbral_alto": 80
      }
    ]
  },
  "tiempo": {
    "jornada": 1,
    "acto": 1,
    "tramo_id": "inicio"
  },
  "presupuesto_temporal": 100,
  "eventos_disponibles": []
}
```

### Configuraciones predefinidas

El proyecto incluye configuraciones de ejemplo en `configs/basic/`:
- `completa.json`: Partida completa de testing
- `minima.json`: Partida mínima para pruebas rápidas

## Desarrollo

### Ejecutar pruebas
```bash
cargo test
```

### Verificar compilación de ejemplos
```bash
cargo check --examples
```

### Formatear código
```bash
cargo fmt
```

### Verificar lint
```bash
cargo clippy
```

## Tecnologías

- **Lenguaje**: Rust 2021 Edition
- **UI**: Slint 1.3 (interfaz gráfica nativa)
- **Terminal**: crossterm + ratatui (para modo terminal)
- **Serialization**: serde + serde_json
- **Logging**: log + env_logger
- **Concurrencia**: tokio, std::sync
- **Red**: reqwest (para integración API)
- **Random**: rand 0.8

## Licencias

Este proyecto usa licencias de código abierto. Consulte el archivo LICENSE para más detalles.

## Contribución

Las contribuciones son bienvenidas. Por favor, abra un issue o envíe un pull request.

## Autor

Álvaro Ruiz
