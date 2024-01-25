/// <https://schema.org/accessibilityAPI>
pub const ACCESSIBILITY_API_PROPERTY_IRI_HTTP: &str = "http://schema.org/accessibilityAPI";
/// <https://schema.org/accessibilityAPI>
pub const ACCESSIBILITY_API_PROPERTY_IRI_HTTPS: &str = "https://schema.org/accessibilityAPI";
/// <https://schema.org/accessibilityAPI>
pub const ACCESSIBILITY_API_PROPERTY_LABEL: &str = "accessibilityAPI";
pub struct AccessibilityApiPropertyIri;
impl PartialEq<&str> for AccessibilityApiPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCESSIBILITY_API_PROPERTY_IRI_HTTP
			|| *other == ACCESSIBILITY_API_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccessibilityApiPropertyIri> for &str {
	fn eq(&self, other: &AccessibilityApiPropertyIri) -> bool {
		*self == ACCESSIBILITY_API_PROPERTY_IRI_HTTP
			|| *self == ACCESSIBILITY_API_PROPERTY_IRI_HTTPS
	}
}
pub struct AccessibilityApiPropertyIriOrLabel;
impl PartialEq<&str> for AccessibilityApiPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccessibilityApiPropertyIri || *other == ACCESSIBILITY_API_PROPERTY_LABEL
	}
}
impl PartialEq<AccessibilityApiPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccessibilityApiPropertyIriOrLabel) -> bool {
		*self == AccessibilityApiPropertyIri || *self == ACCESSIBILITY_API_PROPERTY_LABEL
	}
}
