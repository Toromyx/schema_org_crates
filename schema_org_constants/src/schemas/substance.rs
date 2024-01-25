/// <https://schema.org/Substance>
pub const SUBSTANCE_IRI_HTTP: &str = "http://schema.org/Substance";
/// <https://schema.org/Substance>
pub const SUBSTANCE_IRI_HTTPS: &str = "https://schema.org/Substance";
/// <https://schema.org/Substance>
pub const SUBSTANCE_LABEL: &str = "Substance";
pub struct SubstanceIri;
impl PartialEq<&str> for SubstanceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUBSTANCE_IRI_HTTP || *other == SUBSTANCE_IRI_HTTPS
	}
}
impl PartialEq<SubstanceIri> for &str {
	fn eq(&self, other: &SubstanceIri) -> bool {
		*self == SUBSTANCE_IRI_HTTP || *self == SUBSTANCE_IRI_HTTPS
	}
}
pub struct SubstanceIriOrLabel;
impl PartialEq<&str> for SubstanceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubstanceIri || *other == SUBSTANCE_LABEL
	}
}
impl PartialEq<SubstanceIriOrLabel> for &str {
	fn eq(&self, other: &SubstanceIriOrLabel) -> bool {
		*self == SubstanceIri || *self == SUBSTANCE_LABEL
	}
}
