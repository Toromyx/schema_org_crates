/// <https://schema.org/EvidenceLevelA>
pub const EVIDENCE_LEVEL_A_IRI_HTTP: &str = "http://schema.org/EvidenceLevelA";
/// <https://schema.org/EvidenceLevelA>
pub const EVIDENCE_LEVEL_A_IRI_HTTPS: &str = "https://schema.org/EvidenceLevelA";
/// <https://schema.org/EvidenceLevelA>
pub const EVIDENCE_LEVEL_A_LABEL: &str = "EvidenceLevelA";
pub struct EvidenceLevelAIri;
impl PartialEq<&str> for EvidenceLevelAIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVIDENCE_LEVEL_A_IRI_HTTP || *other == EVIDENCE_LEVEL_A_IRI_HTTPS
	}
}
impl PartialEq<EvidenceLevelAIri> for &str {
	fn eq(&self, other: &EvidenceLevelAIri) -> bool {
		*self == EVIDENCE_LEVEL_A_IRI_HTTP || *self == EVIDENCE_LEVEL_A_IRI_HTTPS
	}
}
pub struct EvidenceLevelAIriOrLabel;
impl PartialEq<&str> for EvidenceLevelAIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EvidenceLevelAIri || *other == EVIDENCE_LEVEL_A_LABEL
	}
}
impl PartialEq<EvidenceLevelAIriOrLabel> for &str {
	fn eq(&self, other: &EvidenceLevelAIriOrLabel) -> bool {
		*self == EvidenceLevelAIri || *self == EVIDENCE_LEVEL_A_LABEL
	}
}
