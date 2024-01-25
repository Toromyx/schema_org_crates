/// <https://schema.org/pregnancyWarning>
pub const PREGNANCY_WARNING_PROPERTY_IRI_HTTP: &str = "http://schema.org/pregnancyWarning";
/// <https://schema.org/pregnancyWarning>
pub const PREGNANCY_WARNING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/pregnancyWarning";
/// <https://schema.org/pregnancyWarning>
pub const PREGNANCY_WARNING_PROPERTY_LABEL: &str = "pregnancyWarning";
pub struct PregnancyWarningPropertyIri;
impl PartialEq<&str> for PregnancyWarningPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREGNANCY_WARNING_PROPERTY_IRI_HTTP
			|| *other == PREGNANCY_WARNING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PregnancyWarningPropertyIri> for &str {
	fn eq(&self, other: &PregnancyWarningPropertyIri) -> bool {
		*self == PREGNANCY_WARNING_PROPERTY_IRI_HTTP
			|| *self == PREGNANCY_WARNING_PROPERTY_IRI_HTTPS
	}
}
pub struct PregnancyWarningPropertyIriOrLabel;
impl PartialEq<&str> for PregnancyWarningPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PregnancyWarningPropertyIri || *other == PREGNANCY_WARNING_PROPERTY_LABEL
	}
}
impl PartialEq<PregnancyWarningPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PregnancyWarningPropertyIriOrLabel) -> bool {
		*self == PregnancyWarningPropertyIri || *self == PREGNANCY_WARNING_PROPERTY_LABEL
	}
}
