/// <https://schema.org/issuedBy>
pub const ISSUED_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/issuedBy";
/// <https://schema.org/issuedBy>
pub const ISSUED_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/issuedBy";
/// <https://schema.org/issuedBy>
pub const ISSUED_BY_PROPERTY_LABEL: &str = "issuedBy";
pub struct IssuedByPropertyIri;
impl PartialEq<&str> for IssuedByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ISSUED_BY_PROPERTY_IRI_HTTP || *other == ISSUED_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IssuedByPropertyIri> for &str {
	fn eq(&self, other: &IssuedByPropertyIri) -> bool {
		*self == ISSUED_BY_PROPERTY_IRI_HTTP || *self == ISSUED_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct IssuedByPropertyIriOrLabel;
impl PartialEq<&str> for IssuedByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IssuedByPropertyIri || *other == ISSUED_BY_PROPERTY_LABEL
	}
}
impl PartialEq<IssuedByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IssuedByPropertyIriOrLabel) -> bool {
		*self == IssuedByPropertyIri || *self == ISSUED_BY_PROPERTY_LABEL
	}
}
