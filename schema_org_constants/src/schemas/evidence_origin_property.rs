/// <https://schema.org/evidenceOrigin>
pub const EVIDENCE_ORIGIN_PROPERTY_IRI_HTTP: &str = "http://schema.org/evidenceOrigin";
/// <https://schema.org/evidenceOrigin>
pub const EVIDENCE_ORIGIN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/evidenceOrigin";
/// <https://schema.org/evidenceOrigin>
pub const EVIDENCE_ORIGIN_PROPERTY_LABEL: &str = "evidenceOrigin";
pub struct EvidenceOriginPropertyIri;
impl PartialEq<&str> for EvidenceOriginPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVIDENCE_ORIGIN_PROPERTY_IRI_HTTP || *other == EVIDENCE_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EvidenceOriginPropertyIri> for &str {
	fn eq(&self, other: &EvidenceOriginPropertyIri) -> bool {
		*self == EVIDENCE_ORIGIN_PROPERTY_IRI_HTTP || *self == EVIDENCE_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
pub struct EvidenceOriginPropertyIriOrLabel;
impl PartialEq<&str> for EvidenceOriginPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EvidenceOriginPropertyIri || *other == EVIDENCE_ORIGIN_PROPERTY_LABEL
	}
}
impl PartialEq<EvidenceOriginPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EvidenceOriginPropertyIriOrLabel) -> bool {
		*self == EvidenceOriginPropertyIri || *self == EVIDENCE_ORIGIN_PROPERTY_LABEL
	}
}
