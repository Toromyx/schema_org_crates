/// <https://schema.org/evidenceLevel>
pub const EVIDENCE_LEVEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/evidenceLevel";
/// <https://schema.org/evidenceLevel>
pub const EVIDENCE_LEVEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/evidenceLevel";
/// <https://schema.org/evidenceLevel>
pub const EVIDENCE_LEVEL_PROPERTY_LABEL: &str = "evidenceLevel";
pub struct EvidenceLevelPropertyIri;
impl PartialEq<&str> for EvidenceLevelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVIDENCE_LEVEL_PROPERTY_IRI_HTTP || *other == EVIDENCE_LEVEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EvidenceLevelPropertyIri> for &str {
	fn eq(&self, other: &EvidenceLevelPropertyIri) -> bool {
		*self == EVIDENCE_LEVEL_PROPERTY_IRI_HTTP || *self == EVIDENCE_LEVEL_PROPERTY_IRI_HTTPS
	}
}
pub struct EvidenceLevelPropertyIriOrLabel;
impl PartialEq<&str> for EvidenceLevelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EvidenceLevelPropertyIri || *other == EVIDENCE_LEVEL_PROPERTY_LABEL
	}
}
impl PartialEq<EvidenceLevelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EvidenceLevelPropertyIriOrLabel) -> bool {
		*self == EvidenceLevelPropertyIri || *self == EVIDENCE_LEVEL_PROPERTY_LABEL
	}
}
