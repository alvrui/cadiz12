use cadiz12::config::{PartidaConfig, PerfilProtagonista, Compromiso, Origen, ClaseSocial, Oficio, AdscripcionPolitica, Temperamento};
use cadiz12::sdk::GeneradorConfig;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Crear perfil del protagonista
    let perfil = PerfilProtagonista {
        origen: Origen::Peninsular,
        clase_social: ClaseSocial::HidalguiaProfesionLetrada,
        oficio: Oficio::JuristaAbogado,
        adscripcion: AdscripcionPolitica::LiberalProgresista,
        temperamento: Temperamento::Prudente,
        compromisos: vec![
            Compromiso {
                id: "deuda_consulado".to_string(),
                tipo: "material".to_string(),
                descripcion: "Deuda con el Consulado de Cadiz".to_string(),
                impacto: "Presion economica constante".to_string(),
            }
        ],
    };

    // Crear generador
    let generador = GeneradorConfig::nuevo();

    // Generar partida
    let config = generador.generar_partida(
        "Mi Partida Cadiz 1812",
        "Partida generada automaticamente",
        perfil,
    )?;

    // Guardar configuracion
    let ruta_salida = "configs/generated/partida_sdk.json";
    config.guardar_como_json(ruta_salida)?;
    println!("Configuracion generada y guardada en: {}", ruta_salida);

    Ok(())
}