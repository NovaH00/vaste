use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkspaceServiceError {
    #[error("workspace not found")]
    WorkspaceNotFound,

    #[error("not the workspace owner")]
    NotOwner,

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}
