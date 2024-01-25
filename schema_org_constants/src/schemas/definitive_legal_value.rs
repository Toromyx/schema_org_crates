/// <https://schema.org/DefinitiveLegalValue>
pub const DEFINITIVE_LEGAL_VALUE_IRI_HTTP: &str = "http://schema.org/DefinitiveLegalValue";
/// <https://schema.org/DefinitiveLegalValue>
pub const DEFINITIVE_LEGAL_VALUE_IRI_HTTPS: &str = "https://schema.org/DefinitiveLegalValue";
/// <https://schema.org/DefinitiveLegalValue>
pub const DEFINITIVE_LEGAL_VALUE_LABEL: &str = "DefinitiveLegalValue";
pub struct DefinitiveLegalValueIri;
impl PartialEq<&str> for DefinitiveLegalValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEFINITIVE_LEGAL_VALUE_IRI_HTTP || *other == DEFINITIVE_LEGAL_VALUE_IRI_HTTPS
	}
}
impl PartialEq<DefinitiveLegalValueIri> for &str {
	fn eq(&self, other: &DefinitiveLegalValueIri) -> bool {
		*self == DEFINITIVE_LEGAL_VALUE_IRI_HTTP || *self == DEFINITIVE_LEGAL_VALUE_IRI_HTTPS
	}
}
pub struct DefinitiveLegalValueIriOrLabel;
impl PartialEq<&str> for DefinitiveLegalValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DefinitiveLegalValueIri || *other == DEFINITIVE_LEGAL_VALUE_LABEL
	}
}
impl PartialEq<DefinitiveLegalValueIriOrLabel> for &str {
	fn eq(&self, other: &DefinitiveLegalValueIriOrLabel) -> bool {
		*self == DefinitiveLegalValueIri || *self == DEFINITIVE_LEGAL_VALUE_LABEL
	}
}
