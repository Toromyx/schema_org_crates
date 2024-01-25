/// <https://schema.org/alcoholWarning>
pub const ALCOHOL_WARNING_PROPERTY_IRI_HTTP: &str = "http://schema.org/alcoholWarning";
/// <https://schema.org/alcoholWarning>
pub const ALCOHOL_WARNING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/alcoholWarning";
/// <https://schema.org/alcoholWarning>
pub const ALCOHOL_WARNING_PROPERTY_LABEL: &str = "alcoholWarning";
pub struct AlcoholWarningPropertyIri;
impl PartialEq<&str> for AlcoholWarningPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALCOHOL_WARNING_PROPERTY_IRI_HTTP || *other == ALCOHOL_WARNING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlcoholWarningPropertyIri> for &str {
	fn eq(&self, other: &AlcoholWarningPropertyIri) -> bool {
		*self == ALCOHOL_WARNING_PROPERTY_IRI_HTTP || *self == ALCOHOL_WARNING_PROPERTY_IRI_HTTPS
	}
}
pub struct AlcoholWarningPropertyIriOrLabel;
impl PartialEq<&str> for AlcoholWarningPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlcoholWarningPropertyIri || *other == ALCOHOL_WARNING_PROPERTY_LABEL
	}
}
impl PartialEq<AlcoholWarningPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlcoholWarningPropertyIriOrLabel) -> bool {
		*self == AlcoholWarningPropertyIri || *self == ALCOHOL_WARNING_PROPERTY_LABEL
	}
}
