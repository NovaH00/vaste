use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeServiceError {
    #[error("node not found")]
    NotFound,

    #[error("node does not belong to this workspace")]
    NotInWorkspace,

    #[error("cannot move a node into one of its own descendants")]
    CircularParent,

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}
