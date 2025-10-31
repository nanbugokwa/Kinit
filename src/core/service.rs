#[derive(Debug, Clone)]
pub struct ServiceDef {
    pub name: String,
    pub command: String,
    pub requires: Vec<String>,
    pub restart: bool,
    pub max_restarts: u32,
}
