/// <https://schema.org/steps>
#[deprecated = "This schema is superseded by <https://schema.org/step>."]
pub const STEPS_PROPERTY_IRI_HTTP: &str = "http://schema.org/steps";
/// <https://schema.org/steps>
#[deprecated = "This schema is superseded by <https://schema.org/step>."]
pub const STEPS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/steps";
/// <https://schema.org/steps>
#[deprecated = "This schema is superseded by <https://schema.org/step>."]
pub const STEPS_PROPERTY_LABEL: &str = "steps";
pub struct StepsPropertyIri;
impl PartialEq<&str> for StepsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STEPS_PROPERTY_IRI_HTTP || *other == STEPS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StepsPropertyIri> for &str {
	fn eq(&self, other: &StepsPropertyIri) -> bool {
		*self == STEPS_PROPERTY_IRI_HTTP || *self == STEPS_PROPERTY_IRI_HTTPS
	}
}
pub struct StepsPropertyIriOrLabel;
impl PartialEq<&str> for StepsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StepsPropertyIri || *other == STEPS_PROPERTY_LABEL
	}
}
impl PartialEq<StepsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StepsPropertyIriOrLabel) -> bool {
		*self == StepsPropertyIri || *self == STEPS_PROPERTY_LABEL
	}
}
