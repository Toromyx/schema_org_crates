/// <https://schema.org/step>
pub const STEP_PROPERTY_IRI_HTTP: &str = "http://schema.org/step";
/// <https://schema.org/step>
pub const STEP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/step";
/// <https://schema.org/step>
pub const STEP_PROPERTY_LABEL: &str = "step";
pub struct StepPropertyIri;
impl PartialEq<&str> for StepPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STEP_PROPERTY_IRI_HTTP || *other == STEP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StepPropertyIri> for &str {
	fn eq(&self, other: &StepPropertyIri) -> bool {
		*self == STEP_PROPERTY_IRI_HTTP || *self == STEP_PROPERTY_IRI_HTTPS
	}
}
pub struct StepPropertyIriOrLabel;
impl PartialEq<&str> for StepPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StepPropertyIri || *other == STEP_PROPERTY_LABEL
	}
}
impl PartialEq<StepPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StepPropertyIriOrLabel) -> bool {
		*self == StepPropertyIri || *self == STEP_PROPERTY_LABEL
	}
}
