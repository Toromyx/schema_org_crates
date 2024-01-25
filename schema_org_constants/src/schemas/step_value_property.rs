/// <https://schema.org/stepValue>
pub const STEP_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/stepValue";
/// <https://schema.org/stepValue>
pub const STEP_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/stepValue";
/// <https://schema.org/stepValue>
pub const STEP_VALUE_PROPERTY_LABEL: &str = "stepValue";
pub struct StepValuePropertyIri;
impl PartialEq<&str> for StepValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STEP_VALUE_PROPERTY_IRI_HTTP || *other == STEP_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StepValuePropertyIri> for &str {
	fn eq(&self, other: &StepValuePropertyIri) -> bool {
		*self == STEP_VALUE_PROPERTY_IRI_HTTP || *self == STEP_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct StepValuePropertyIriOrLabel;
impl PartialEq<&str> for StepValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StepValuePropertyIri || *other == STEP_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<StepValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &StepValuePropertyIriOrLabel) -> bool {
		*self == StepValuePropertyIri || *self == STEP_VALUE_PROPERTY_LABEL
	}
}
