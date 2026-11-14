use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Route {
    pub name: String,
    pub namespace: String,
    pub kind: String,
    pub hostname: String,
}
