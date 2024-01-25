/// <https://schema.org/accessibilityFeature>
pub const ACCESSIBILITY_FEATURE_PROPERTY_IRI_HTTP: &str = "http://schema.org/accessibilityFeature";
/// <https://schema.org/accessibilityFeature>
pub const ACCESSIBILITY_FEATURE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/accessibilityFeature";
/// <https://schema.org/accessibilityFeature>
pub const ACCESSIBILITY_FEATURE_PROPERTY_LABEL: &str = "accessibilityFeature";
pub struct AccessibilityFeaturePropertyIri;
impl PartialEq<&str> for AccessibilityFeaturePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCESSIBILITY_FEATURE_PROPERTY_IRI_HTTP
			|| *other == ACCESSIBILITY_FEATURE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccessibilityFeaturePropertyIri> for &str {
	fn eq(&self, other: &AccessibilityFeaturePropertyIri) -> bool {
		*self == ACCESSIBILITY_FEATURE_PROPERTY_IRI_HTTP
			|| *self == ACCESSIBILITY_FEATURE_PROPERTY_IRI_HTTPS
	}
}
pub struct AccessibilityFeaturePropertyIriOrLabel;
impl PartialEq<&str> for AccessibilityFeaturePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccessibilityFeaturePropertyIri || *other == ACCESSIBILITY_FEATURE_PROPERTY_LABEL
	}
}
impl PartialEq<AccessibilityFeaturePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccessibilityFeaturePropertyIriOrLabel) -> bool {
		*self == AccessibilityFeaturePropertyIri || *self == ACCESSIBILITY_FEATURE_PROPERTY_LABEL
	}
}
