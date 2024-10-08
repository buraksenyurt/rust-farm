use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, ApiComponent, JsonSchema)]
pub struct Challenge {
    pub id: i32,
    pub serial_code: String,
    pub title: String,
    pub duration: u32,
    pub duration_type: DurationType,
    pub purpose: String,
    pub details: String,
    pub stakeholders: String,
    pub expected_outputs: String,
    pub benefits: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ApiComponent, JsonSchema)]
pub enum DurationType {
    Day,
    Week,
    Month,
}
