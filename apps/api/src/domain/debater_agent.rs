#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebateSide {
    Affirmative,
    Negative,
}

#[derive(Debug, Clone)]
pub struct DebaterAgent {
    pub id: String,
    pub side: DebateSide,
    pub role: String,
    pub display_name: String,
    pub persona: String,
    pub objective: String,
    pub style: String,
    pub model: String,
}
