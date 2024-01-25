/// <https://schema.org/asin>
pub const ASIN_PROPERTY_IRI_HTTP: &str = "http://schema.org/asin";
/// <https://schema.org/asin>
pub const ASIN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/asin";
/// <https://schema.org/asin>
pub const ASIN_PROPERTY_LABEL: &str = "asin";
pub struct AsinPropertyIri;
impl PartialEq<&str> for AsinPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASIN_PROPERTY_IRI_HTTP || *other == ASIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AsinPropertyIri> for &str {
	fn eq(&self, other: &AsinPropertyIri) -> bool {
		*self == ASIN_PROPERTY_IRI_HTTP || *self == ASIN_PROPERTY_IRI_HTTPS
	}
}
pub struct AsinPropertyIriOrLabel;
impl PartialEq<&str> for AsinPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AsinPropertyIri || *other == ASIN_PROPERTY_LABEL
	}
}
impl PartialEq<AsinPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AsinPropertyIriOrLabel) -> bool {
		*self == AsinPropertyIri || *self == ASIN_PROPERTY_LABEL
	}
}
