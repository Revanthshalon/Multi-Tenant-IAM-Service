use crate::entities::tenants::Tenant;
use crate::errors::user_management::UserManagementErrors;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct TenantRepository {
    pool: PgPool,
}

impl TenantRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_new_tenant(
        &self,
        name: &str,
        parent_id: Option<Uuid>,
    ) -> Result<Tenant, UserManagementErrors> {
        todo!()
    }
}
