use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ModError {
    #[error("{0}")]
    IO(std::io::Error),

    #[error("{0}")]
    Serde(serde_json::Error),

    #[error("Provided Path is not a valid loader mod")]
    InvalidModPath,

    #[error("Mod with UUID '{0}' cannot be found in instance")]
    NotFound(Uuid),
}
