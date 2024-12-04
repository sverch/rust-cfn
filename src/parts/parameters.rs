use indexmap::IndexMap;
use serde::Deserialize;

/// Declares input values that you can use to pass values into this
/// cloudformation stack.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Parameters(IndexMap<String, ::serde_json::Value>);

impl Parameters {
    /// Get the parameter identified by the logical id.
    ///
    /// If the parameter does not exist, or has a different type, an error is
    /// returned.
    pub fn get(&self, id: &str) -> Result<Parameter, crate::Error> {
        self.0
            .get(id)
            .ok_or_else(|| {
                crate::Error::new(
                    crate::ErrorKind::NotFound,
                    format_args!("output with logical id {} not found", id),
                )
            })
            .and_then(|inner| {
                Parameter::deserialize(inner)
                    .map_err(|err| crate::Error::new(crate::ErrorKind::Serialization, err))
            })
    }

    /// Checks if the parameter identified by the logical id exists.
    pub fn has(&self, id: &str) -> bool {
        self.0.contains_key(id)
    }

    /// Insert a parameter with the provided logical id.
    pub fn set(&mut self, id: &str, output: Parameter) {
        let inner = ::serde_json::to_value(output).unwrap();
        self.0.insert(id.to_owned(), inner);
    }
}

/// Parameter input to a CloudFormation template.
#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    /// A value of the appropriate type for the template to use if no value is
    /// specified when a stack is created.
    #[serde(rename = "Default")]
    pub default: Option<String>,

    /// A string of up to 4000 characters that describes the parameter.
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// The data type for the parameter.
    #[serde(rename = "Type")]
    pub r#type: String,
}
