/// <https://schema.org/ParentalSupport>
pub const PARENTAL_SUPPORT_IRI_HTTP: &str = "http://schema.org/ParentalSupport";
/// <https://schema.org/ParentalSupport>
pub const PARENTAL_SUPPORT_IRI_HTTPS: &str = "https://schema.org/ParentalSupport";
/// <https://schema.org/ParentalSupport>
pub const PARENTAL_SUPPORT_LABEL: &str = "ParentalSupport";
pub struct ParentalSupportIri;
impl PartialEq<&str> for ParentalSupportIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARENTAL_SUPPORT_IRI_HTTP || *other == PARENTAL_SUPPORT_IRI_HTTPS
	}
}
impl PartialEq<ParentalSupportIri> for &str {
	fn eq(&self, other: &ParentalSupportIri) -> bool {
		*self == PARENTAL_SUPPORT_IRI_HTTP || *self == PARENTAL_SUPPORT_IRI_HTTPS
	}
}
pub struct ParentalSupportIriOrLabel;
impl PartialEq<&str> for ParentalSupportIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParentalSupportIri || *other == PARENTAL_SUPPORT_LABEL
	}
}
impl PartialEq<ParentalSupportIriOrLabel> for &str {
	fn eq(&self, other: &ParentalSupportIriOrLabel) -> bool {
		*self == ParentalSupportIri || *self == PARENTAL_SUPPORT_LABEL
	}
}
