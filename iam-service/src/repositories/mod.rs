use sqlx::PgPool;
use crate::repositories::tenant::TenantRepository;

mod tenant;

#[derive(Clone)]
pub struct RepositoryContainer {
    pub tenant_repository: TenantRepository
}

impl RepositoryContainer {
    pub fn new(pool: PgPool) -> Self {
        let tenant_repository = TenantRepository::new(pool.clone());
        Self {
            tenant_repository
        }
    }
}