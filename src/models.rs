#![allow(unused)]
#![allow(clippy::all)]

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Condition {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaAction {
    pub id: i64,
    pub formula_id: i64,
    pub formula_named_action_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaAlsoKnown {
    pub id: i64,
    pub name: String,
    pub formula_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaCondition {
    pub id: i64,
    pub formula_id: i64,
    pub condition_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaContraindication {
    pub id: i64,
    pub formula_id: i64,
    pub contraindication: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaHerbAction {
    pub id: i64,
    pub formula_herb_id: i64,
    pub formula_named_action_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaHerb {
    pub id: i64,
    pub formula_id: i64,
    pub herb_id: i64,
    pub dosage: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaInteraction {
    pub id: i64,
    pub formula_id: i64,
    pub interaction: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaManifestationSymptom {
    pub id: i64,
    pub formula_manifestation_id: i64,
    pub symptom_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaManifestation {
    pub id: i64,
    pub formula_id: i64,
    pub tongue: Option<String>,
    pub tongue_coat: Option<String>,
    pub pulse: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaModificationSet {
    pub id: i64,
    pub formula_id: i64,
    pub purpose: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaModification {
    pub id: i64,
    pub formula_modification_set_id: i64,
    pub herb_id: i64,
    pub modification: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaNamedAction {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaNote {
    pub id: i64,
    pub formula_id: i64,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaSyndrome {
    pub id: i64,
    pub formula_id: i64,
    pub syndrome_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Formula {
    pub id: i64,
    pub name: String,
    pub pinyin: Option<String>,
    pub hanzi: Option<String>,
    pub english: Option<String>,
    pub common_english: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbActionAnnotation {
    pub id: i64,
    pub herb_action_id: i64,
    pub annotation: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbActionIndication {
    pub id: i64,
    pub herb_action_id: i64,
    pub indication: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbActionType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbAction {
    pub id: i64,
    pub herb_id: i64,
    pub herb_action_type_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbCategory {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbCombinationHerb {
    pub id: i64,
    pub herb_id: i64,
    pub herb_combination_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbCombinationUseCase {
    pub id: i64,
    pub herb_combination_id: i64,
    pub use_case: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbCombination {
    pub id: i64,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbDosageType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbDosage {
    pub id: i64,
    pub herb_id: i64,
    pub herb_dosage_type_id: i64,
    pub dosage: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbNote {
    pub id: i64,
    pub herb_id: i64,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbProperty {
    pub id: i64,
    pub herb_id: i64,
    pub herb_property_type_id: i64,
    pub precedence_type_id: i64,
    pub property: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbPropertyType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbWarningType {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HerbWarning {
    pub id: i64,
    pub herb_id: i64,
    pub herb_warning_type_id: i64,
    pub warning: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Herb {
    pub id: i64,
    pub name: String,
    pub herb_category_id: i64,
    pub pinyin: Option<String>,
    pub hanzi: Option<String>,
    pub latin: Option<String>,
    pub pharm_latin: Option<String>,
    pub common_english: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrecedenceType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Symptom {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Syndrome {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


