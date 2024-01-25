/// <https://schema.org/LegalService>
pub const LEGAL_SERVICE_IRI_HTTP: &str = "http://schema.org/LegalService";
/// <https://schema.org/LegalService>
pub const LEGAL_SERVICE_IRI_HTTPS: &str = "https://schema.org/LegalService";
/// <https://schema.org/LegalService>
pub const LEGAL_SERVICE_LABEL: &str = "LegalService";
pub struct LegalServiceIri;
impl PartialEq<&str> for LegalServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGAL_SERVICE_IRI_HTTP || *other == LEGAL_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<LegalServiceIri> for &str {
	fn eq(&self, other: &LegalServiceIri) -> bool {
		*self == LEGAL_SERVICE_IRI_HTTP || *self == LEGAL_SERVICE_IRI_HTTPS
	}
}
pub struct LegalServiceIriOrLabel;
impl PartialEq<&str> for LegalServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegalServiceIri || *other == LEGAL_SERVICE_LABEL
	}
}
impl PartialEq<LegalServiceIriOrLabel> for &str {
	fn eq(&self, other: &LegalServiceIriOrLabel) -> bool {
		*self == LegalServiceIri || *self == LEGAL_SERVICE_LABEL
	}
}
