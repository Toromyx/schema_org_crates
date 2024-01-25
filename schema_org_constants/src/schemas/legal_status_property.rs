/// <https://schema.org/legalStatus>
pub const LEGAL_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/legalStatus";
/// <https://schema.org/legalStatus>
pub const LEGAL_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/legalStatus";
/// <https://schema.org/legalStatus>
pub const LEGAL_STATUS_PROPERTY_LABEL: &str = "legalStatus";
pub struct LegalStatusPropertyIri;
impl PartialEq<&str> for LegalStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGAL_STATUS_PROPERTY_IRI_HTTP || *other == LEGAL_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegalStatusPropertyIri> for &str {
	fn eq(&self, other: &LegalStatusPropertyIri) -> bool {
		*self == LEGAL_STATUS_PROPERTY_IRI_HTTP || *self == LEGAL_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct LegalStatusPropertyIriOrLabel;
impl PartialEq<&str> for LegalStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegalStatusPropertyIri || *other == LEGAL_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<LegalStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegalStatusPropertyIriOrLabel) -> bool {
		*self == LegalStatusPropertyIri || *self == LEGAL_STATUS_PROPERTY_LABEL
	}
}
