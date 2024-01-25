/// <https://schema.org/DefinedTerm>
pub const DEFINED_TERM_IRI_HTTP: &str = "http://schema.org/DefinedTerm";
/// <https://schema.org/DefinedTerm>
pub const DEFINED_TERM_IRI_HTTPS: &str = "https://schema.org/DefinedTerm";
/// <https://schema.org/DefinedTerm>
pub const DEFINED_TERM_LABEL: &str = "DefinedTerm";
pub struct DefinedTermIri;
impl PartialEq<&str> for DefinedTermIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEFINED_TERM_IRI_HTTP || *other == DEFINED_TERM_IRI_HTTPS
	}
}
impl PartialEq<DefinedTermIri> for &str {
	fn eq(&self, other: &DefinedTermIri) -> bool {
		*self == DEFINED_TERM_IRI_HTTP || *self == DEFINED_TERM_IRI_HTTPS
	}
}
pub struct DefinedTermIriOrLabel;
impl PartialEq<&str> for DefinedTermIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DefinedTermIri || *other == DEFINED_TERM_LABEL
	}
}
impl PartialEq<DefinedTermIriOrLabel> for &str {
	fn eq(&self, other: &DefinedTermIriOrLabel) -> bool {
		*self == DefinedTermIri || *self == DEFINED_TERM_LABEL
	}
}
