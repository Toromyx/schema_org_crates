/// <https://schema.org/actionAccessibilityRequirement>
pub const ACTION_ACCESSIBILITY_REQUIREMENT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/actionAccessibilityRequirement";
/// <https://schema.org/actionAccessibilityRequirement>
pub const ACTION_ACCESSIBILITY_REQUIREMENT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/actionAccessibilityRequirement";
/// <https://schema.org/actionAccessibilityRequirement>
pub const ACTION_ACCESSIBILITY_REQUIREMENT_PROPERTY_LABEL: &str = "actionAccessibilityRequirement";
pub struct ActionAccessibilityRequirementPropertyIri;
impl PartialEq<&str> for ActionAccessibilityRequirementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTION_ACCESSIBILITY_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *other == ACTION_ACCESSIBILITY_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActionAccessibilityRequirementPropertyIri> for &str {
	fn eq(&self, other: &ActionAccessibilityRequirementPropertyIri) -> bool {
		*self == ACTION_ACCESSIBILITY_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *self == ACTION_ACCESSIBILITY_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct ActionAccessibilityRequirementPropertyIriOrLabel;
impl PartialEq<&str> for ActionAccessibilityRequirementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActionAccessibilityRequirementPropertyIri
			|| *other == ACTION_ACCESSIBILITY_REQUIREMENT_PROPERTY_LABEL
	}
}
impl PartialEq<ActionAccessibilityRequirementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActionAccessibilityRequirementPropertyIriOrLabel) -> bool {
		*self == ActionAccessibilityRequirementPropertyIri
			|| *self == ACTION_ACCESSIBILITY_REQUIREMENT_PROPERTY_LABEL
	}
}
