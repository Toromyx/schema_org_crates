/// <https://schema.org/requirements>
#[deprecated = "This schema is superseded by <https://schema.org/softwareRequirements>."]
pub const REQUIREMENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/requirements";
/// <https://schema.org/requirements>
#[deprecated = "This schema is superseded by <https://schema.org/softwareRequirements>."]
pub const REQUIREMENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/requirements";
/// <https://schema.org/requirements>
#[deprecated = "This schema is superseded by <https://schema.org/softwareRequirements>."]
pub const REQUIREMENTS_PROPERTY_LABEL: &str = "requirements";
pub struct RequirementsPropertyIri;
impl PartialEq<&str> for RequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REQUIREMENTS_PROPERTY_IRI_HTTP || *other == REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RequirementsPropertyIri> for &str {
	fn eq(&self, other: &RequirementsPropertyIri) -> bool {
		*self == REQUIREMENTS_PROPERTY_IRI_HTTP || *self == REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct RequirementsPropertyIriOrLabel;
impl PartialEq<&str> for RequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RequirementsPropertyIri || *other == REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<RequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RequirementsPropertyIriOrLabel) -> bool {
		*self == RequirementsPropertyIri || *self == REQUIREMENTS_PROPERTY_LABEL
	}
}
