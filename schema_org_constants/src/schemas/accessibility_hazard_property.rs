/// <https://schema.org/accessibilityHazard>
pub const ACCESSIBILITY_HAZARD_PROPERTY_IRI_HTTP: &str = "http://schema.org/accessibilityHazard";
/// <https://schema.org/accessibilityHazard>
pub const ACCESSIBILITY_HAZARD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/accessibilityHazard";
/// <https://schema.org/accessibilityHazard>
pub const ACCESSIBILITY_HAZARD_PROPERTY_LABEL: &str = "accessibilityHazard";
pub struct AccessibilityHazardPropertyIri;
impl PartialEq<&str> for AccessibilityHazardPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCESSIBILITY_HAZARD_PROPERTY_IRI_HTTP
			|| *other == ACCESSIBILITY_HAZARD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccessibilityHazardPropertyIri> for &str {
	fn eq(&self, other: &AccessibilityHazardPropertyIri) -> bool {
		*self == ACCESSIBILITY_HAZARD_PROPERTY_IRI_HTTP
			|| *self == ACCESSIBILITY_HAZARD_PROPERTY_IRI_HTTPS
	}
}
pub struct AccessibilityHazardPropertyIriOrLabel;
impl PartialEq<&str> for AccessibilityHazardPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccessibilityHazardPropertyIri || *other == ACCESSIBILITY_HAZARD_PROPERTY_LABEL
	}
}
impl PartialEq<AccessibilityHazardPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccessibilityHazardPropertyIriOrLabel) -> bool {
		*self == AccessibilityHazardPropertyIri || *self == ACCESSIBILITY_HAZARD_PROPERTY_LABEL
	}
}
