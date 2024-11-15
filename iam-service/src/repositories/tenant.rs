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
        // Check if name exists
        if let Ok(_) = self.get_tenant_by_name(name).await {
            return Err(UserManagementErrors::UserAlreadyExists);
        }

        // Begin transaction
        let mut tx = self.pool.begin().await?;

        let tenant_result = sqlx::query_as::<_, Tenant>(
            r#"
            INSERT INTO tenants (name, parent_id)
            VALUES ($1, $2)
            RETURNING
                id,
                name,
                parent_id,
                created_at,
                updated_at
            "#,
        )
        .bind(name)
        .bind(parent_id)
        .fetch_one(&mut *tx)
        .await;

        let tenant = match tenant_result {
            Ok(result) => {
                tx.commit().await;
                result
            }
            Err(e) => {
                // TODO: Tracing Info
                tx.rollback().await;
                return Err(UserManagementErrors::DatabaseErrors(e));
            }
        };

        Ok(tenant)
    }

    pub async fn get_tenant_by_name(
        &self,
        tenanat_name: &str,
    ) -> Result<Tenant, UserManagementErrors> {
        let tenanat_result = sqlx::query_as::<_, Tenant>(
            r#"
        SELECT
            id,
            name,
            parent_id,
            created_at,
            updated_at
        FROM
            tenants
        WHERE
            name = $1
        "#,
        )
        .bind(tenanat_name)
        .fetch_one(&self.pool)
        .await;

        match tenanat_result {
            Ok(tenant) => Ok(tenant),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Err(UserManagementErrors::UserNotFound),
                _ => {
                    // TODO: log tracing info
                    Err(UserManagementErrors::DatabaseErrors(e))
                }
            },
        }
    }
}
