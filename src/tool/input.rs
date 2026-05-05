use schemars::JsonSchema;
use serde::de::DeserializeOwned;

/// Input parameters to a tool.
pub trait ToolInput: DeserializeOwned + JsonSchema {}
