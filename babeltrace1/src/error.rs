use crate::TraceHandleId;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to create context")]
    ContextCreation,

    #[error("failed to add trace")]
    TraceAdd,

    #[error("no trace with id {0} does not exist")]
    TraceRemove(TraceHandleId),

    #[error("invalid trace path")]
    InvalidTracePath,

    #[error("failed to get event scope")]
    GetEventScope,

    #[error("invalid value for field")]
    InvalidValueForField,

    #[error("invalid definition")]
    InvalidDefinition,

    #[error("invalid timestamp present")]
    InvalidTimestamp,

    #[error("unknown field {0}")]
    UnknownField(String),
}
