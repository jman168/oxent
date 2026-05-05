use serde::Serialize;

/// Output parameters of a tool.
pub trait ToolOutput: Serialize {}
