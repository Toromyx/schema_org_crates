/// <https://schema.org/accessibilityControl>
pub const ACCESSIBILITY_CONTROL_PROPERTY_IRI_HTTP: &str = "http://schema.org/accessibilityControl";
/// <https://schema.org/accessibilityControl>
pub const ACCESSIBILITY_CONTROL_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/accessibilityControl";
/// <https://schema.org/accessibilityControl>
pub const ACCESSIBILITY_CONTROL_PROPERTY_LABEL: &str = "accessibilityControl";
pub struct AccessibilityControlPropertyIri;
impl PartialEq<&str> for AccessibilityControlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCESSIBILITY_CONTROL_PROPERTY_IRI_HTTP
			|| *other == ACCESSIBILITY_CONTROL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccessibilityControlPropertyIri> for &str {
	fn eq(&self, other: &AccessibilityControlPropertyIri) -> bool {
		*self == ACCESSIBILITY_CONTROL_PROPERTY_IRI_HTTP
			|| *self == ACCESSIBILITY_CONTROL_PROPERTY_IRI_HTTPS
	}
}
pub struct AccessibilityControlPropertyIriOrLabel;
impl PartialEq<&str> for AccessibilityControlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccessibilityControlPropertyIri || *other == ACCESSIBILITY_CONTROL_PROPERTY_LABEL
	}
}
impl PartialEq<AccessibilityControlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccessibilityControlPropertyIriOrLabel) -> bool {
		*self == AccessibilityControlPropertyIri || *self == ACCESSIBILITY_CONTROL_PROPERTY_LABEL
	}
}
