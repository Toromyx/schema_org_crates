/// <https://schema.org/issuedThrough>
pub const ISSUED_THROUGH_PROPERTY_IRI_HTTP: &str = "http://schema.org/issuedThrough";
/// <https://schema.org/issuedThrough>
pub const ISSUED_THROUGH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/issuedThrough";
/// <https://schema.org/issuedThrough>
pub const ISSUED_THROUGH_PROPERTY_LABEL: &str = "issuedThrough";
pub struct IssuedThroughPropertyIri;
impl PartialEq<&str> for IssuedThroughPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ISSUED_THROUGH_PROPERTY_IRI_HTTP || *other == ISSUED_THROUGH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IssuedThroughPropertyIri> for &str {
	fn eq(&self, other: &IssuedThroughPropertyIri) -> bool {
		*self == ISSUED_THROUGH_PROPERTY_IRI_HTTP || *self == ISSUED_THROUGH_PROPERTY_IRI_HTTPS
	}
}
pub struct IssuedThroughPropertyIriOrLabel;
impl PartialEq<&str> for IssuedThroughPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IssuedThroughPropertyIri || *other == ISSUED_THROUGH_PROPERTY_LABEL
	}
}
impl PartialEq<IssuedThroughPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IssuedThroughPropertyIriOrLabel) -> bool {
		*self == IssuedThroughPropertyIri || *self == ISSUED_THROUGH_PROPERTY_LABEL
	}
}
