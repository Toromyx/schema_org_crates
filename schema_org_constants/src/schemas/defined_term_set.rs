/// <https://schema.org/DefinedTermSet>
pub const DEFINED_TERM_SET_IRI_HTTP: &str = "http://schema.org/DefinedTermSet";
/// <https://schema.org/DefinedTermSet>
pub const DEFINED_TERM_SET_IRI_HTTPS: &str = "https://schema.org/DefinedTermSet";
/// <https://schema.org/DefinedTermSet>
pub const DEFINED_TERM_SET_LABEL: &str = "DefinedTermSet";
pub struct DefinedTermSetIri;
impl PartialEq<&str> for DefinedTermSetIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEFINED_TERM_SET_IRI_HTTP || *other == DEFINED_TERM_SET_IRI_HTTPS
	}
}
impl PartialEq<DefinedTermSetIri> for &str {
	fn eq(&self, other: &DefinedTermSetIri) -> bool {
		*self == DEFINED_TERM_SET_IRI_HTTP || *self == DEFINED_TERM_SET_IRI_HTTPS
	}
}
pub struct DefinedTermSetIriOrLabel;
impl PartialEq<&str> for DefinedTermSetIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DefinedTermSetIri || *other == DEFINED_TERM_SET_LABEL
	}
}
impl PartialEq<DefinedTermSetIriOrLabel> for &str {
	fn eq(&self, other: &DefinedTermSetIriOrLabel) -> bool {
		*self == DefinedTermSetIri || *self == DEFINED_TERM_SET_LABEL
	}
}
