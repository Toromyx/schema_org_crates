/// <https://schema.org/breastfeedingWarning>
pub const BREASTFEEDING_WARNING_PROPERTY_IRI_HTTP: &str = "http://schema.org/breastfeedingWarning";
/// <https://schema.org/breastfeedingWarning>
pub const BREASTFEEDING_WARNING_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/breastfeedingWarning";
/// <https://schema.org/breastfeedingWarning>
pub const BREASTFEEDING_WARNING_PROPERTY_LABEL: &str = "breastfeedingWarning";
pub struct BreastfeedingWarningPropertyIri;
impl PartialEq<&str> for BreastfeedingWarningPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BREASTFEEDING_WARNING_PROPERTY_IRI_HTTP
			|| *other == BREASTFEEDING_WARNING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BreastfeedingWarningPropertyIri> for &str {
	fn eq(&self, other: &BreastfeedingWarningPropertyIri) -> bool {
		*self == BREASTFEEDING_WARNING_PROPERTY_IRI_HTTP
			|| *self == BREASTFEEDING_WARNING_PROPERTY_IRI_HTTPS
	}
}
pub struct BreastfeedingWarningPropertyIriOrLabel;
impl PartialEq<&str> for BreastfeedingWarningPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BreastfeedingWarningPropertyIri || *other == BREASTFEEDING_WARNING_PROPERTY_LABEL
	}
}
impl PartialEq<BreastfeedingWarningPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BreastfeedingWarningPropertyIriOrLabel) -> bool {
		*self == BreastfeedingWarningPropertyIri || *self == BREASTFEEDING_WARNING_PROPERTY_LABEL
	}
}
