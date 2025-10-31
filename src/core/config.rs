use std::fs;
use crate::core::service::ServiceDef;

#[cfg(feature = "serde_json")]
pub fn load(path: &str) -> Result<Vec<ServiceDef>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let services: Vec<ServiceDef> = serde_json::from_str(&content)?;
    Ok(services)
}

#[cfg(not(feature = "serde_json"))]
pub fn load(_path: &str) -> Result<Vec<ServiceDef>, Box<dyn std::error::Error>> {
    Ok(vec![])
}
