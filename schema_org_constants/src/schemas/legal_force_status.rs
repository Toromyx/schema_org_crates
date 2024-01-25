/// <https://schema.org/LegalForceStatus>
pub const LEGAL_FORCE_STATUS_IRI_HTTP: &str = "http://schema.org/LegalForceStatus";
/// <https://schema.org/LegalForceStatus>
pub const LEGAL_FORCE_STATUS_IRI_HTTPS: &str = "https://schema.org/LegalForceStatus";
/// <https://schema.org/LegalForceStatus>
pub const LEGAL_FORCE_STATUS_LABEL: &str = "LegalForceStatus";
pub struct LegalForceStatusIri;
impl PartialEq<&str> for LegalForceStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGAL_FORCE_STATUS_IRI_HTTP || *other == LEGAL_FORCE_STATUS_IRI_HTTPS
	}
}
impl PartialEq<LegalForceStatusIri> for &str {
	fn eq(&self, other: &LegalForceStatusIri) -> bool {
		*self == LEGAL_FORCE_STATUS_IRI_HTTP || *self == LEGAL_FORCE_STATUS_IRI_HTTPS
	}
}
pub struct LegalForceStatusIriOrLabel;
impl PartialEq<&str> for LegalForceStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegalForceStatusIri || *other == LEGAL_FORCE_STATUS_LABEL
	}
}
impl PartialEq<LegalForceStatusIriOrLabel> for &str {
	fn eq(&self, other: &LegalForceStatusIriOrLabel) -> bool {
		*self == LegalForceStatusIri || *self == LEGAL_FORCE_STATUS_LABEL
	}
}
