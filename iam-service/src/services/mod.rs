use std::sync::Arc;
use crate::repositories::RepositoryContainer;
use crate::services::user_management::UserManagementService;

mod user_management;

pub struct ServiceContainer {
    pub user_management_service: UserManagementService
}

impl ServiceContainer {
    pub fn new(repository_container: RepositoryContainer) -> Arc<Self> {
        let user_management_service = UserManagementService::new(repository_container.clone());
        let service_container = Self {
            user_management_service
        };

        Arc::new(service_container)
    }
}