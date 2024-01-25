/// <https://schema.org/correction>
pub const CORRECTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/correction";
/// <https://schema.org/correction>
pub const CORRECTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/correction";
/// <https://schema.org/correction>
pub const CORRECTION_PROPERTY_LABEL: &str = "correction";
pub struct CorrectionPropertyIri;
impl PartialEq<&str> for CorrectionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CORRECTION_PROPERTY_IRI_HTTP || *other == CORRECTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CorrectionPropertyIri> for &str {
	fn eq(&self, other: &CorrectionPropertyIri) -> bool {
		*self == CORRECTION_PROPERTY_IRI_HTTP || *self == CORRECTION_PROPERTY_IRI_HTTPS
	}
}
pub struct CorrectionPropertyIriOrLabel;
impl PartialEq<&str> for CorrectionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CorrectionPropertyIri || *other == CORRECTION_PROPERTY_LABEL
	}
}
impl PartialEq<CorrectionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CorrectionPropertyIriOrLabel) -> bool {
		*self == CorrectionPropertyIri || *self == CORRECTION_PROPERTY_LABEL
	}
}
