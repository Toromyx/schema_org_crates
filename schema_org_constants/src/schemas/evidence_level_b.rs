/// <https://schema.org/EvidenceLevelB>
pub const EVIDENCE_LEVEL_B_IRI_HTTP: &str = "http://schema.org/EvidenceLevelB";
/// <https://schema.org/EvidenceLevelB>
pub const EVIDENCE_LEVEL_B_IRI_HTTPS: &str = "https://schema.org/EvidenceLevelB";
/// <https://schema.org/EvidenceLevelB>
pub const EVIDENCE_LEVEL_B_LABEL: &str = "EvidenceLevelB";
pub struct EvidenceLevelBIri;
impl PartialEq<&str> for EvidenceLevelBIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVIDENCE_LEVEL_B_IRI_HTTP || *other == EVIDENCE_LEVEL_B_IRI_HTTPS
	}
}
impl PartialEq<EvidenceLevelBIri> for &str {
	fn eq(&self, other: &EvidenceLevelBIri) -> bool {
		*self == EVIDENCE_LEVEL_B_IRI_HTTP || *self == EVIDENCE_LEVEL_B_IRI_HTTPS
	}
}
pub struct EvidenceLevelBIriOrLabel;
impl PartialEq<&str> for EvidenceLevelBIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EvidenceLevelBIri || *other == EVIDENCE_LEVEL_B_LABEL
	}
}
impl PartialEq<EvidenceLevelBIriOrLabel> for &str {
	fn eq(&self, other: &EvidenceLevelBIriOrLabel) -> bool {
		*self == EvidenceLevelBIri || *self == EVIDENCE_LEVEL_B_LABEL
	}
}
