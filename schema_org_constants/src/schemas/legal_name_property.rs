/// <https://schema.org/legalName>
pub const LEGAL_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/legalName";
/// <https://schema.org/legalName>
pub const LEGAL_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/legalName";
/// <https://schema.org/legalName>
pub const LEGAL_NAME_PROPERTY_LABEL: &str = "legalName";
pub struct LegalNamePropertyIri;
impl PartialEq<&str> for LegalNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGAL_NAME_PROPERTY_IRI_HTTP || *other == LEGAL_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegalNamePropertyIri> for &str {
	fn eq(&self, other: &LegalNamePropertyIri) -> bool {
		*self == LEGAL_NAME_PROPERTY_IRI_HTTP || *self == LEGAL_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct LegalNamePropertyIriOrLabel;
impl PartialEq<&str> for LegalNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegalNamePropertyIri || *other == LEGAL_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<LegalNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegalNamePropertyIriOrLabel) -> bool {
		*self == LegalNamePropertyIri || *self == LEGAL_NAME_PROPERTY_LABEL
	}
}
