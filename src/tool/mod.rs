//! LLM callable tools.

use async_openai::types::assistants::FunctionObject;
use schemars::schema_for;

mod input;
pub use input::ToolInput;

mod output;
pub use output::ToolOutput;

/// An LLM callable tool.
pub trait Tool {
    type Input: ToolInput;
    type Output: ToolOutput;

    /// Evaluates the tool for a given input.
    fn evaluate(&mut self, input: Self::Input) -> Self::Output;

    /// Returns the name of the tool.
    fn name(&self) -> String;

    /// Returns a description of the tool.
    fn description(&self) -> Option<String>;

    /// Whether to enforce strict mode for the function call.
    fn strict(&self) -> Option<bool>;

    /// Returns the tool description which is fed to the LLM.
    fn tool_description(&self) -> serde_json::Result<FunctionObject> {
        let schema = schema_for!(Self::Input);
        let schema_value = serde_json::to_value(schema)?;

        Ok(FunctionObject {
            name: self.name(),
            description: self.description(),
            parameters: Some(schema_value),
            strict: self.strict(),
        })
    }

    /// Evaluates the tool for a given JSON input and returns a JSON output.
    fn evaluate_json(&mut self, input: &str) -> serde_json::Result<String> {
        let input: Self::Input = serde_json::from_str(input)?;
        let output = self.evaluate(input);
        serde_json::to_string(&output)
    }
}
