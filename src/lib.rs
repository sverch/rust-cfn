#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_debug_implementations)]

//! This crate provides type-safe representations for AWS CloudFormation templates,
//! resources and properties.

extern crate indexmap;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod error;
#[macro_use]
mod codec;
pub mod aws;
mod parts;
mod types;

pub use crate::error::{Error, ErrorKind};
pub use crate::parts::*;
pub use crate::types::*;

pub mod json {
    //! Types for raw JSON values.
    pub use serde_json::{Number, Value};
}

/// Represents an AWS CloudFormation template.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Template {
    #[serde(
        rename = "Description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    description: Option<String>,
    #[serde(rename = "Resources", default)]
    resources: Resources,
    #[serde(rename = "Outputs", default)]
    outputs: Outputs,
    #[serde(rename = "Parameters", default)]
    parameters: Parameters,
}

impl Template {
    /// Get the template description as a reference.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Get the template description as a mutable reference.
    pub fn description_mut(&mut self) -> &mut Option<String> {
        &mut self.description
    }

    /// Get a reference to the resources defined in the template.
    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    /// Get a mutable reference to the resources defined in the template.
    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    /// Get a reference to the outputs defined in the template.
    pub fn outputs(&self) -> &Outputs {
        &self.outputs
    }

    /// Get a mutable reference to the outputs defined in the template.
    pub fn outputs_mut(&mut self) -> &mut Outputs {
        &mut self.outputs
    }

    /// Get a reference to the parameters defined in the template.
    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }

    /// Get a mutable reference to the parameters defined in the template.
    pub fn parameters_mut(&mut self) -> &mut Parameters {
        &mut self.parameters
    }
}

impl Template {
    /// Deserialize a AWS CloudFormation template in JSON format.
    pub fn from_json<T: AsRef<str>>(input: T) -> Result<Template, crate::Error> {
        serde_json::from_str(input.as_ref())
            .map_err(|err| crate::Error::new(crate::ErrorKind::Serialization, err))
    }

    /// Serialize the template into an AWS CloudFormation template in JSON format.
    pub fn to_json(&self) -> Result<String, crate::Error> {
        serde_json::to_string(self)
            .map_err(|err| crate::Error::new(crate::ErrorKind::Serialization, err))
    }
}

/// Trait for stack resources, such as an Amazon Elastic Compute Cloud instance or an Amazon Simple Storage Service bucket.
pub trait Resource: Sized + private::Sealed {
    /// Uniquely identifies the resource type.
    const TYPE: &'static str;
    /// Type that represents the set of properties the resource can be configured with.
    type Properties: private::Properties<Self>;

    /// Get a reference to the properties on the resource.
    fn properties(&self) -> &Self::Properties;
    /// Get a mutable reference to the properties on the resource.
    fn properties_mut(&mut self) -> &mut Self::Properties;
    /// Get a reference to the explicit resource dependencies.
    fn depends_on(&self) -> &Option<crate::DependsOn>;
    /// Get a mutable reference to the explicit resource dependencies.
    fn depends_on_mut(&mut self) -> &mut Option<crate::DependsOn>;
}

mod private {
    pub trait Sealed {}

    pub trait Properties<R>: Into<R> + ::serde::Serialize + ::serde::de::DeserializeOwned {}
    impl<P, R> Properties<R> for P where P: Into<R> + ::serde::Serialize + ::serde::de::DeserializeOwned {}
}

#[cfg(test)]
mod tests {
    use super::Template;
    use serde_json::to_value;

    #[test]
    fn deserialize_empty_template() {
        let tpl = Template::from_json("{}").unwrap();
        assert!(tpl.description().is_none())
    }

    #[test]
    fn serialize_empty_template() {
        let tpl = Template::default();
        let val = to_value(&tpl).unwrap();
        let obj = val.as_object().unwrap();
        assert_eq!(2, obj.len());
        assert_eq!(false, obj.contains_key("Description"));
        assert_eq!(true, obj.contains_key("Resources"));
        assert_eq!(true, obj.contains_key("Outputs"));
    }
}
