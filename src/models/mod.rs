use loco_rs::errors;
use sea_orm::DbErr;

pub mod _entities;
pub mod notes;
pub mod posts;
pub mod posts_tags;
pub mod tags;
pub mod users;
pub mod series;
pub mod posts_series;

#[derive(thiserror::Error, Debug)]
pub enum ModelsError {
    #[error(transparent)]
    FrameworkError(#[from]  loco_rs::model::ModelError),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl From<DbErr> for ModelsError {
    fn from(err: DbErr) -> Self {
        Self::FrameworkError(err.into())
    }
}

impl From<ModelsError> for errors::Error {
    fn from(err: ModelsError) -> Self {
        tracing::error!("ModelsError: {:?}", err);
        match err {
            ModelsError::FrameworkError(err) => err.into(),
            ModelsError::PermissionDenied(msg) => Self::Unauthorized(msg),
        }
    }
}
