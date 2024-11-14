use crate::repositories::RepositoryContainer;

pub struct UserManagementService {
    repository_container: RepositoryContainer
}

impl UserManagementService {
    pub fn new(repository_container: RepositoryContainer) -> Self {
        Self { repository_container }
    }
}