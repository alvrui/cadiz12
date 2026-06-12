# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-06-12

### Features

- **UI Slint Integration**: Complete Slint 1.3 UI implementation with ScrollView for medidores and eventos
- **Event Resolution**: UI buttons for events (0-9) and options (1-9) connected to motor API
- **Real-time Updates**: Automatic UI refresh after event resolution
- **Error Handling**: Validation and error messages in footer for all user interactions

### UI Components

- **Header**: Game title "CADIZ 1812"
- **Info Bar**: Jornada, Acto, Tramo, Presupuesto display
- **Medidores Panel**: 5 medidores with nombre, valor, tendencia, umbrales
- **Eventos Panel**: 10 event buttons with title and temporal cost
- **Footer**: Message text and option buttons 1-9

### Bug Fixes

- Fixed Slint 1.3 compatibility issues (no ListView, no complex expressions)
- Resolved callback registration for event resolution
- Corrected type handling for Window references in Slint 1.3

### Documentation

- Added comprehensive inline documentation for UI components
- Created callback-based architecture for event handling

## [0.1.0] - 2026-06-11

### Features

- **Motor API**: Complete game engine with state management
- **Configuration**: PartidaConfig for game setup
- **Event System**: Event generation and resolution
- **Protagonist State**: Medidores (influencia, relacional, reputacion, coherencia, recursos)
- **World State**: Tiempo tracking (jornada, acto, tramo)
- **Terminal UI**: Basic ratatui-based terminal interface

### Engine Modules

- **m1_estado_mundo**: World state management (tiempo, espacios)
- **m2_estado_protagonista**: Protagonist state and medidores
- **m3_medidores**: Medidor calculations and thresholds
- **m4_generador_eventos**: Event generation from templates
- **m5_bucle_jornada**: Game loop and event resolution
- **m6_memoria**: Game state persistence
- **m7_api**: Public API for game engine

### API Endpoints

- `GET /estado_jornada`: Returns complete journey state
- `GET /evento/{evento_id}`: Returns event details
- `POST /resolver_evento`: Resolves event with selected option
- `GET /estado_personaje`: Returns protagonist state

### Configuration

- Perfil initial setup (origen, clase_social, oficio, adscripcion, temperamento)
- Event templates with temporal cost and priority
- Medidor configurations with thresholds

### Testing

- Comprehensive test suite for motor API
- Integration tests for game flow
- Configuration validation tests

### Documentation

- Module-level documentation for all engine components
- Example configurations for testing
- API documentation for all public methods
