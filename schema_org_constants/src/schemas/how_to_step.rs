/// <https://schema.org/HowToStep>
pub const HOW_TO_STEP_IRI_HTTP: &str = "http://schema.org/HowToStep";
/// <https://schema.org/HowToStep>
pub const HOW_TO_STEP_IRI_HTTPS: &str = "https://schema.org/HowToStep";
/// <https://schema.org/HowToStep>
pub const HOW_TO_STEP_LABEL: &str = "HowToStep";
pub struct HowToStepIri;
impl PartialEq<&str> for HowToStepIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_TO_STEP_IRI_HTTP || *other == HOW_TO_STEP_IRI_HTTPS
	}
}
impl PartialEq<HowToStepIri> for &str {
	fn eq(&self, other: &HowToStepIri) -> bool {
		*self == HOW_TO_STEP_IRI_HTTP || *self == HOW_TO_STEP_IRI_HTTPS
	}
}
pub struct HowToStepIriOrLabel;
impl PartialEq<&str> for HowToStepIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowToStepIri || *other == HOW_TO_STEP_LABEL
	}
}
impl PartialEq<HowToStepIriOrLabel> for &str {
	fn eq(&self, other: &HowToStepIriOrLabel) -> bool {
		*self == HowToStepIri || *self == HOW_TO_STEP_LABEL
	}
}
