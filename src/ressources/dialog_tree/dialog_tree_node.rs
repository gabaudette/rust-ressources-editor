#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogTreeNode {
    pub id: String,
    
    pub text: String,

    #[serde(default)]
    pub speaker_id: Option<String>,

    #[serde(default)]
    pub is_end: bool,

    #[serde(default)]
    pub next_node_id: Option<String>,

    #[serde(default)]
    pub next_event_id: Option<String>,
}