use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterTenant {
    pub name: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize())]
pub struct GetTenant {
    pub id: Uuid,
}
