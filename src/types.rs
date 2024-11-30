use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Tags {
    RiscV,
    Linux,
    Windows,
    Kernel,
    // Add more about computer science
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub(crate) id: Uuid,              // UUID for unique identification
    pub(crate) title: String,         // Title of the note
    pub(crate) content: String,       // Markdown text
    pub(crate) created_at: u64,       // Unix epoch time
    pub(crate) updated_at: u64,       // Unix epoch time
    pub(crate) tags: Vec<Tags>,       // Array of tags
}