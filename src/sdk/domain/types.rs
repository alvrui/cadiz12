// Domain types for Cádiz 1812 SDK v3.0
// This file contains all closed enum types forming the formal SDK contract

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

// Re-export ID types from ids module
pub use super::ids::*;

/// Error type for domain parsing operations
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DomainParseError {
    #[error("Unknown variant: {0}")]
    UnknownVariant(String),
    #[error("Empty string")]
    EmptyString,
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Value out of range: {0} (expected {1})")]
    OutOfRange(String, String),
    #[error("Duplicate ID: {0}")]
    DuplicateId(String),
    #[error("Missing reference: {0}")]
    MissingReference(String),
}

// Existing types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum FaccionId {
    Liberal, Absolutista, Clero, Militar, Pueblo, Nobleza, Burguesia, Extranjero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Origen { Local, Foraneo, Noble, Plebeyo, Militar, Clerigo, Comerciante, Artesano }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ClaseSocial {
    AltaNobleza, BajaNobleza, Burguesia, CleroAlto, CleroBajo, MilitarAlto, MilitarBajo,
    Artesano, Comerciante, Campesino, Obrero, Mendigo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Oficio {
    Politico, Militar, Clerigo, Comerciante, Artesano, Abogado, Medico, Periodista,
    Escritor, Obrero, Campesino, Sirviente, Espia, Contrabandista, Noble,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Adscripcion {
    Liberal, Absolutista, Moderado, Radical, Conservador, Reformista, Neutral, Oportunista,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Temperamento {
    Impulsivo, Reflexivo, Agresivo, Pacifista, Honesto, Astuto, Leal, Traitor,
    Optimista, Pesimista, Carismatico, Timido,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PosicionFormalId {
    Diputado, Senador, Ministro, General, Almirante, Obispo, Alcalde, Juez,
    Abogado, Periodista, Comerciante, Artesano, Campesino, Noble, Sirviente,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Visibilidad { Publico, Privado, Secreto, Oculto }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TrayectoriaMoral {
    Heroico, Villano, Antiheroe, Neutral, Oportunista, Idealista, Pragmatico, Corrupto, Redimido, Caido,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TipoEvento {
    Politico, Personal, Militar, Social, Economico, Religioso, Judicial, Diplomatico, Cultural, Urgente,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum FaseCrisis { Inicio, Desarrollo, Climax, Resolucion, Consecuencia }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TipoCrisis {
    Politica, Social, Economica, Militar, Religiosa, Personal, Institucional, Internacional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EspacioId {
    Cortes, Calle, Taberna, Iglesia, Cuartel, Puerto, Mercado, Palacio, Casa, Prision, Hospital, Universidad, Plaza,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EstadoRelacion {
    Aliado, Amigo, Conocido, Neutral, Desconfiado, Enemigo, Rival, Mentor, Protegido, Familiar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum MedidorId {
    Influencia, Reputacion, Moral, Salud, Riqueza, Conocimiento, Lealtad, Miedo, Esperanza, Ira,
}

// New enums
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ScriptElementCategory {
    Protagonist, Antagonist, Secondary, ThemeEvent, Finale, Scenario, Procedure, SocialPressure, DramaticResource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum HistoricalScope { PlausibleDocumented, PlausibleInferred, ExceptionalButVerisimilar, Discarded }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TimeSlice { Y1805_1808, Y1809, Y1810, Y1811, Y1812, Y1813, Y1814, Y1815_1816 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Act { Act1, Act2, Act3, Act4 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum StakesAxis {
    Personal, Political, Urban, Institutional, Imperial, Moral, Economic, Religious, Military, Media,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Tone {
    Festive, Satirical, Anxious, Solemn, Intimate, Conspiratorial, Patriotic, Sordid,
    Tragic, Ambiguous, Tense, Polemical, Funereal, Resilient, Combative, Compassionate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ChainRole { Seed, Complication, Escalation, Crisis, Revelation, Resolution, Aftermath }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Repeatability { Unique, Rare, ControlledRecurring, Serial }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum VisibilityProfile { None, Discreet, Public }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum InformationProfile { PublicFact, PlausibleRumor, ConfidentialDocument, NetworkSecret, DeliberateAmbiguity }

pub type CompatibilityScore = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ArcSlotRuleType { RequireTag, BlockTag, Faction, Profile, Tone, Act, TimeWindow }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RuleOperator { Eq, Neq, Gte, Lte, Contains, NotContains, In, NotIn }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ProfileAxis { Oficio, Origen, ClaseSocial, Adscripcion, Temperamento }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum FitKind { Ideal, Possible, Forced }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum FactionAffinityKind { Ally, Hostile, Dependent, Opportunist }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ReputationPolarity { Gains, Loses, Tensions }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TagRequirementKind { Require, Block }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum OutcomeTargetKind { Meter, Relationship, Reputation, Tag, Crisis }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ThematicAxis {
    Sovereignty, Religion, Press, Debt, Equality, Justice, Freedom, Order, Progress, Tradition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum BindingKind { Requires, Favors, Forbids, Decorates }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ArcKind { Political, Relational, Reputational, Mixed }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ArcSlotKind { Element, EventTemplate, Selector }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ResolutionKind { PartialVictory, Cost, Rupture, FullVictory, Compromise, Failure }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EventFamilyId { A, B, C, D, E, F, S1, S2, S3, S4, S5 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EventFunctionId { Pressure, Opportunity, Exposure, Transition, Crisis, Resolution }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TimeCost { Instant, Short, Medium, Long }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum StaminaCost { None, Low, Medium, High }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ConsequenceKind { Positive, Negative, Mixed, Neutral }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum NpcCriterion { Affine, Rival, Recent, RandomWeighted, ByFaction, ByPosition }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ChainKind { Narrative, Emergent, Sidechain }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ChainState { Open, Active, Suspended, Resolved, Failed }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ArcState { Dormant, Active, Suspended, Resolved, Failed }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ArcInstanceSlotState { Pending, Offered, Resolved, Skipped }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ChainNodeState { Generated, Offered, Chosen, Resolved, Expired }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum NarrativeState { Dormant, Active, Suspended, Resolved, Failed }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EventInstanceState { Generated, Offered, Chosen, Resolved, Expired }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum GlobalState { TenseNormality, PreCrisis, OpenCrisis, PostCrisisAftermath, PublicCelebration, LatentRepression }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PublicVisibilityLevel { Unknown, Emerging, RecognizableFigure, HighlyExposed }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum MoralTrajectory { Opportunist, Coherent, Ambiguous, Reliable, Feared, Indispensable }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SpaceClimate { Calm, Saturated, Nervous, Watched, Empty, Effervescent }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum CrisisPhase { Signal, Outbreak, ReactionPeriod, Resolution, Aftermath }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RelationshipLevel { Unknown, Contact, Ally, IntimatePolitical, Rival, Enemy }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum RelationshipState { Stable, Resentful, Tense, Grateful, Broken, UnderReview }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SceneTemplateType { AInstitutionalSession, BUrbanEncounter, CPrivateVisit, DDocumentReading, EPublicCrisis, FPersonalConsequence }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PressureSeriesType { MorningMail, AfternoonEdition, CallToPosition, PendingCommitmentPressure, RumorWithConsequence }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ProcedureKind {
    DebatePlenary, TechnicalCommission, EmergencySession, HonorSession, DecreeVote,
    PriorPositioningCall, PrivateNegotiation, PetitionSubmission, DocumentReading,
    PressPublication, PressDenunciation, StrategicLeak, StrategicDelay, AmendmentProposal,
    SecretVoteRequest, SafeConductProcessing, HousingAssignment, HealthDeclaration,
    NeighborhoodRelief, SignatureCirculation,
}

pub type Acto = String;
pub type Jornada = u32;
pub type Weight = f32;
pub type MeterDeltaValue = i16;
pub type RelationshipDeltaValue = i16;
pub type ReputationDeltaValue = i16;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompatibilitySet {
    pub protagonist_vs_theme: HashMap<ProtagonistId, HashMap<ThemeId, CompatibilityScore>>,
    pub scenario_vs_theme: HashMap<ScenarioId, HashMap<ThemeId, CompatibilityScore>>,
    pub antagonist_vs_theme: HashMap<AntagonistId, HashMap<ThemeId, CompatibilityScore>>,
    pub secondary_vs_theme: HashMap<SecondaryId, HashMap<ThemeId, CompatibilityScore>>,
    pub procedure_vs_theme: HashMap<ProcedureId, HashMap<ThemeId, CompatibilityScore>>,
}

#[derive(Debug, Clone, Copy)]
pub struct ScoringWeights {
    pub w_protagonist: Weight,
    pub w_scenario: Weight,
    pub w_antagonist: Weight,
    pub w_time: Weight,
    pub w_act: Weight,
    pub w_faction: Weight,
    pub w_state: Weight,
    pub w_novelty: Weight,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            w_protagonist: 3.0, w_scenario: 2.0, w_antagonist: 1.5, w_time: 2.0,
            w_act: 1.5, w_faction: 1.5, w_state: 2.0, w_novelty: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptElementBase {
    pub id: ElementId, pub category: ScriptElementCategory, pub label: String, pub description: String,
    pub historical_scope: HistoricalScope, pub time_window: Vec<TimeSlice>, pub act_bias: Vec<Act>,
    pub faction_vectors: Vec<FaccionId>, pub space_vectors: Vec<EspacioId>, pub stakes_axis: Vec<StakesAxis>,
    pub tone: Tone, pub chain_role: ChainRole, pub repeatability: Repeatability,
    pub meter_affinity: Vec<MeterType>, pub visibility_profile: VisibilityProfile,
    pub information_profile: InformationProfile, pub compatibility_tags: Vec<TagId>,
    pub blocking_tags: Vec<TagId>, pub unlock_tags: Vec<TagId>, pub generated_tags: Vec<TagId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtagonistArchetype {
    pub base: ScriptElementBase, pub eligible_profiles: Vec<ProfileAxis>, pub eligible_positions: Vec<PosicionFormalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeDef {
    pub narrative_id: NarrativeId, pub slug: String, pub title: String, pub summary: String,
    pub thematic_axis: ThematicAxis, pub default_priority: u8, pub act_start: Act, pub act_end: Act,
    pub repeatable: bool, pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcTemplate {
    pub arc_template_id: ArcTemplateId, pub narrative_id: NarrativeId, pub title: String, pub description: String,
    pub arc_kind: ArcKind, pub act_open: Act, pub act_close: Act, pub min_chain_length: u8, pub max_chain_length: u8,
    pub default_resolution_kind: ResolutionKind, pub can_fail_forward: bool, pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcSlotTemplate {
    pub arc_slot_id: ArcSlotId, pub arc_template_id: ArcTemplateId, pub slot_order: u8,
    pub chain_role: ChainRole, pub slot_kind: ArcSlotKind, pub required: bool,
    pub min_options: u8, pub max_options: u8, pub tone_bias: Option<Tone>, pub crisis_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcSlotRule {
    pub arc_slot_rule_id: String, pub arc_slot_id: ArcSlotId, pub rule_type: ArcSlotRuleType,
    pub operator: RuleOperator, pub value: String, pub weight_delta: Option<i16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTemplate {
    pub template_id: EventTemplateId, pub family_id: EventFamilyId, pub function_id: EventFunctionId,
    pub weight_base: i16, pub cooldown_days: u16, pub time_cost: TimeCost, pub stamina_cost: StaminaCost,
    pub visibility_output: VisibilityProfile, pub info_incomplete: bool, pub consequence_kind: ConsequenceKind,
    pub crisis_compatible: bool, pub npc_criterion: NpcCriterion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateElementBinding {
    pub template_id: EventTemplateId, pub element_id: ElementId, pub binding_kind: BindingKind, pub weight_delta: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstance {
    pub id: EventInstanceId, pub scene_template: SceneTemplateType, pub pressure_series: Option<PressureSeriesType>,
    pub main_theme: ThemeId, pub satellite_themes: Vec<ThemeId>, pub protagonist: ProtagonistId,
    pub antagonist: Option<AntagonistId>, pub secondaries: Vec<SecondaryId>, pub scenario: ScenarioId,
    pub procedure: ProcedureId, pub dramatic_resources: Vec<ResourceId>, pub scheduled_time: TimeSlice,
    pub stakes_axis: Vec<StakesAxis>, pub tone: Tone, pub expected_meter_effects: Vec<MeterDeltaValue>,
    pub generated_tags: Vec<TagId>, pub trace: SelectionTrace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeBindings {
    pub theme_id: ThemeId, pub secondary_ids: Vec<SecondaryId>, pub procedure_ids: Vec<ProcedureId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub component: String, pub value: f32, pub weight: f32, pub weighted_value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeCandidateTrace {
    pub theme_id: ThemeId, pub score: f32, pub breakdown: Vec<ScoreBreakdown>,
    pub rejected: bool, pub rejection_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionTrace {
    pub candidate_themes: Vec<ThemeCandidateTrace>, pub chosen_theme: ThemeId,
    pub chosen_secondary_reasons: Vec<String>, pub chosen_procedure_reasons: Vec<String>,
    pub rejected_reasons: Vec<String>, pub final_score_breakdown: Vec<ScoreBreakdown>,
}

impl fmt::Display for SelectionTrace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Selection Trace:")?;
        writeln!(f, "  Chosen Theme: {}", self.chosen_theme.0)?;
        writeln!(f, "  Secondary Reasons: {:?}", self.chosen_secondary_reasons)?;
        writeln!(f, "  Procedure Reasons: {:?}", self.chosen_procedure_reasons)?;
        writeln!(f, "  Rejected Reasons: {:?}", self.rejected_reasons)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MeterValue {
    pub meter_type: MeterType, pub value: i32, pub min: i32, pub max: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RelationshipValue {
    pub target_id: ElementId, pub level: RelationshipLevel, pub state: RelationshipState, pub value: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub global_state: GlobalState, pub current_act: Act, pub current_tramo: TimeSlice,
    pub crisis_phase: Option<CrisisPhase>, pub active_tags: Vec<TagId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtagonistState {
    pub protagonist_id: ProtagonistId, pub position: PosicionFormalId, pub visibility: PublicVisibilityLevel,
    pub moral_trajectory: MoralTrajectory, pub meters: HashMap<MeterType, MeterValue>,
    pub reputation: HashMap<ReputationGroupId, i16>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionState {
    pub session_id: SessionId, pub current_jornada: Jornada, pub world: WorldState,
    pub protagonist: ProtagonistState, pub active_narratives: Vec<NarrativeInstanceId>,
    pub active_arcs: Vec<ArcInstanceId>, pub active_chains: Vec<ChainId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    DuplicateId(String), InvalidRange(String, String), MissingReference(String, String),
    InvalidCompatibilityScore(CompatibilityScore), MissingTimeWindow(ElementId), MissingActBias(ElementId),
    MissingThemeBindings(ThemeId), IncompatibleProcedure(ProcedureId), DiscardedThemeInCatalog(ThemeId),
    MissingTemplateRules(EventTemplateId),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::DuplicateId(id) => write!(f, "Duplicate ID: {}", id),
            ValidationError::InvalidRange(field, expected) => write!(f, "Invalid range for {}: expected {}", field, expected),
            ValidationError::MissingReference(entity, id) => write!(f, "Missing reference: {} {}", entity, id),
            ValidationError::InvalidCompatibilityScore(score) => write!(f, "Invalid compatibility score: {} (must be 0-5)", score),
            ValidationError::MissingTimeWindow(id) => write!(f, "Missing time window for element: {}", id.0),
            ValidationError::MissingActBias(id) => write!(f, "Missing act bias for element: {}", id.0),
            ValidationError::MissingThemeBindings(theme_id) => write!(f, "Missing theme bindings for theme: {}", theme_id.0),
            ValidationError::IncompatibleProcedure(proc_id) => write!(f, "Incompatible procedure: {}", proc_id.0),
            ValidationError::DiscardedThemeInCatalog(theme_id) => write!(f, "Discarded theme in catalog: {}", theme_id.0),
            ValidationError::MissingTemplateRules(template_id) => write!(f, "Missing template rules for: {}", template_id.0),
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeterDeltaEffect { pub meter_type: MeterType, pub delta: MeterDeltaValue }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipDeltaEffect {
    pub target_id: ElementId, pub level_delta: Option<RelationshipLevel>, pub state_delta: Option<RelationshipState>,
    pub value_delta: RelationshipDeltaValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationDeltaEffect {
    pub group_id: ReputationGroupId, pub delta: ReputationDeltaValue, pub polarity: ReputationPolarity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOutcomePrototype {
    pub meter_deltas: Vec<MeterDeltaEffect>, pub relationship_deltas: Vec<RelationshipDeltaEffect>,
    pub reputation_deltas: Vec<ReputationDeltaEffect>, pub add_tags: Vec<TagId>, pub remove_tags: Vec<TagId>,
    pub unlock_elements: Vec<ElementId>, pub lock_elements: Vec<ElementId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstanceOutcome {
    pub outcome_id: OutcomeId, pub event_instance_id: EventInstanceId, pub target_kind: OutcomeTargetKind,
    pub target_id: Option<String>, pub delta_value: i16, pub execute_on_jornada: Jornada, pub applied: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_enums() {
        assert_eq!("liberal".parse::<FaccionId>().unwrap(), FaccionId::Liberal);
        assert_eq!("protagonist".parse::<ScriptElementCategory>().unwrap(), ScriptElementCategory::Protagonist);
    }
    #[test] fn test_weights() {
        let w = ScoringWeights::default();
        assert_eq!(w.w_protagonist, 3.0);
    }
}
