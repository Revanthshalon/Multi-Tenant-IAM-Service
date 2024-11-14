use crate::repositories::RepositoryContainer;
use crate::services::user_management::UserManagementService;

mod user_management;

pub struct ServiceContainer {
    pub user_management_service: UserManagementService
}

impl ServiceContainer {
    pub fn new(repository_container: RepositoryContainer) -> Self {
        let user_management_service = UserManagementService::new(repository_container.clone());
        Self {
            user_management_service
        }
    }
}