/// <https://schema.org/LegalValueLevel>
pub const LEGAL_VALUE_LEVEL_IRI_HTTP: &str = "http://schema.org/LegalValueLevel";
/// <https://schema.org/LegalValueLevel>
pub const LEGAL_VALUE_LEVEL_IRI_HTTPS: &str = "https://schema.org/LegalValueLevel";
/// <https://schema.org/LegalValueLevel>
pub const LEGAL_VALUE_LEVEL_LABEL: &str = "LegalValueLevel";
pub struct LegalValueLevelIri;
impl PartialEq<&str> for LegalValueLevelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGAL_VALUE_LEVEL_IRI_HTTP || *other == LEGAL_VALUE_LEVEL_IRI_HTTPS
	}
}
impl PartialEq<LegalValueLevelIri> for &str {
	fn eq(&self, other: &LegalValueLevelIri) -> bool {
		*self == LEGAL_VALUE_LEVEL_IRI_HTTP || *self == LEGAL_VALUE_LEVEL_IRI_HTTPS
	}
}
pub struct LegalValueLevelIriOrLabel;
impl PartialEq<&str> for LegalValueLevelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegalValueLevelIri || *other == LEGAL_VALUE_LEVEL_LABEL
	}
}
impl PartialEq<LegalValueLevelIriOrLabel> for &str {
	fn eq(&self, other: &LegalValueLevelIriOrLabel) -> bool {
		*self == LegalValueLevelIri || *self == LEGAL_VALUE_LEVEL_LABEL
	}
}
