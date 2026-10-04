//! Open Recipe Format (ORF) types.
//!
//! Mirrors the zod schema in `.pi/skills/import-recipe/scripts/orf.ts`.
//! Unknown fields are ignored on deserialization (matching zod's silent
//! stripping), except that `X-<field>` extension keys are captured in
//! `extra` maps so no data is lost.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// `{amount, unit}` pair — amount is how many of `unit`. Used for
/// ingredient amounts, yields, and (wrapped differently) oven temp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub amount: f64,
    pub unit: String,
}

/// A substitute ingredient. Same format as a regular ingredient, minus the
/// `substitutions` field (no nesting).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Substitution {
    pub ingredient: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<Amount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usda_num: Option<String>,
    /// Non-spec extension fields (`X-<field>`).
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// One food item and how much of it to use.
///
/// Ingredients are listed in the order in which they are used; an item may
/// appear multiple times at different quantities. With multiple yields,
/// `amounts` holds one entry per yield, in the same order as `yields`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub ingredient: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<Amount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usda_num: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<Vec<Substitution>>,
    /// Non-spec extension fields (`X-<field>`).
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// HACCP guidance for a step. Exactly one of the two fields is set.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Haccp {
    /// A relevant HACCP guideline.
    ControlPoint(String),
    /// One that is critical to the safety outcome.
    CriticalControlPoint(String),
}

/// One preparation step, in order. `step` is the only required field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub step: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub haccp: Option<Haccp>,
    /// "Bench notes" — professional asides about the step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    /// Non-spec extension fields (`X-<field>`).
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Starting oven temperature, if the oven is used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvenTemp {
    pub amount: f64,
    pub unit: TempUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TempUnit {
    C,
    F,
}

/// Convection oven setting. Absent means `Off`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OvenFan {
    #[default]
    Off,
    Low,
    High,
}

/// Book source, if the recipe was originally pulled from a book.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Non-spec extension fields (`X-<field>`).
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A recipe in Open Recipe Format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// The name of this recipe.
    pub recipe_name: String,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<Step>,
    /// How much the recipe makes. Normally one entry; one per yield when
    /// multiple yields are stored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yields: Option<Vec<Amount>>,
    /// URL the recipe was copied from (its official hosted URL, if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// The original author of the recipe — NOT the person who entered it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_authors: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_book: Option<SourceBook>,
    /// Identifier for the company/software + unique recipe id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipe_uuid: Option<String>,
    /// Recipe-level notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Convection oven setting. Absent means `Off`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oven_fan: Option<OvenFan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oven_temp: Option<Vec<OvenTemp>>,
    /// Overall oven time for the dish (e.g. "45 minutes").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oven_time: Option<String>,
    /// Non-spec extension fields (`X-<field>`), e.g. `X-image`.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Recipe {
    /// Path/URL of the recipe's image, from the `X-image` extension field.
    pub fn image(&self) -> Option<&str> {
        self.extra.get("X-image").and_then(|v| v.as_str())
    }

    /// Attribution for the recipe's image, from the `X-image-attribution` extension field.
    pub fn image_attribution(&self) -> Option<&str> {
        self.extra
            .get("X-image-attribution")
            .and_then(|v| v.as_str())
    }
}
