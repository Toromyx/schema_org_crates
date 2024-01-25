/// <https://schema.org/EvidenceLevelC>
pub const EVIDENCE_LEVEL_C_IRI_HTTP: &str = "http://schema.org/EvidenceLevelC";
/// <https://schema.org/EvidenceLevelC>
pub const EVIDENCE_LEVEL_C_IRI_HTTPS: &str = "https://schema.org/EvidenceLevelC";
/// <https://schema.org/EvidenceLevelC>
pub const EVIDENCE_LEVEL_C_LABEL: &str = "EvidenceLevelC";
pub struct EvidenceLevelCIri;
impl PartialEq<&str> for EvidenceLevelCIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVIDENCE_LEVEL_C_IRI_HTTP || *other == EVIDENCE_LEVEL_C_IRI_HTTPS
	}
}
impl PartialEq<EvidenceLevelCIri> for &str {
	fn eq(&self, other: &EvidenceLevelCIri) -> bool {
		*self == EVIDENCE_LEVEL_C_IRI_HTTP || *self == EVIDENCE_LEVEL_C_IRI_HTTPS
	}
}
pub struct EvidenceLevelCIriOrLabel;
impl PartialEq<&str> for EvidenceLevelCIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EvidenceLevelCIri || *other == EVIDENCE_LEVEL_C_LABEL
	}
}
impl PartialEq<EvidenceLevelCIriOrLabel> for &str {
	fn eq(&self, other: &EvidenceLevelCIriOrLabel) -> bool {
		*self == EvidenceLevelCIri || *self == EVIDENCE_LEVEL_C_LABEL
	}
}
