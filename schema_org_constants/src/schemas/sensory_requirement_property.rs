/// <https://schema.org/sensoryRequirement>
pub const SENSORY_REQUIREMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/sensoryRequirement";
/// <https://schema.org/sensoryRequirement>
pub const SENSORY_REQUIREMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sensoryRequirement";
/// <https://schema.org/sensoryRequirement>
pub const SENSORY_REQUIREMENT_PROPERTY_LABEL: &str = "sensoryRequirement";
pub struct SensoryRequirementPropertyIri;
impl PartialEq<&str> for SensoryRequirementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SENSORY_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *other == SENSORY_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SensoryRequirementPropertyIri> for &str {
	fn eq(&self, other: &SensoryRequirementPropertyIri) -> bool {
		*self == SENSORY_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *self == SENSORY_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SensoryRequirementPropertyIriOrLabel;
impl PartialEq<&str> for SensoryRequirementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SensoryRequirementPropertyIri || *other == SENSORY_REQUIREMENT_PROPERTY_LABEL
	}
}
impl PartialEq<SensoryRequirementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SensoryRequirementPropertyIriOrLabel) -> bool {
		*self == SensoryRequirementPropertyIri || *self == SENSORY_REQUIREMENT_PROPERTY_LABEL
	}
}
