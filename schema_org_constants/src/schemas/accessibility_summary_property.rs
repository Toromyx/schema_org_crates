/// <https://schema.org/accessibilitySummary>
pub const ACCESSIBILITY_SUMMARY_PROPERTY_IRI_HTTP: &str = "http://schema.org/accessibilitySummary";
/// <https://schema.org/accessibilitySummary>
pub const ACCESSIBILITY_SUMMARY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/accessibilitySummary";
/// <https://schema.org/accessibilitySummary>
pub const ACCESSIBILITY_SUMMARY_PROPERTY_LABEL: &str = "accessibilitySummary";
pub struct AccessibilitySummaryPropertyIri;
impl PartialEq<&str> for AccessibilitySummaryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCESSIBILITY_SUMMARY_PROPERTY_IRI_HTTP
			|| *other == ACCESSIBILITY_SUMMARY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccessibilitySummaryPropertyIri> for &str {
	fn eq(&self, other: &AccessibilitySummaryPropertyIri) -> bool {
		*self == ACCESSIBILITY_SUMMARY_PROPERTY_IRI_HTTP
			|| *self == ACCESSIBILITY_SUMMARY_PROPERTY_IRI_HTTPS
	}
}
pub struct AccessibilitySummaryPropertyIriOrLabel;
impl PartialEq<&str> for AccessibilitySummaryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccessibilitySummaryPropertyIri || *other == ACCESSIBILITY_SUMMARY_PROPERTY_LABEL
	}
}
impl PartialEq<AccessibilitySummaryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccessibilitySummaryPropertyIriOrLabel) -> bool {
		*self == AccessibilitySummaryPropertyIri || *self == ACCESSIBILITY_SUMMARY_PROPERTY_LABEL
	}
}
