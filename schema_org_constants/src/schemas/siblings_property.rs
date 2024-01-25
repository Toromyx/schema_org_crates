/// <https://schema.org/siblings>
#[deprecated = "This schema is superseded by <https://schema.org/sibling>."]
pub const SIBLINGS_PROPERTY_IRI_HTTP: &str = "http://schema.org/siblings";
/// <https://schema.org/siblings>
#[deprecated = "This schema is superseded by <https://schema.org/sibling>."]
pub const SIBLINGS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/siblings";
/// <https://schema.org/siblings>
#[deprecated = "This schema is superseded by <https://schema.org/sibling>."]
pub const SIBLINGS_PROPERTY_LABEL: &str = "siblings";
pub struct SiblingsPropertyIri;
impl PartialEq<&str> for SiblingsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIBLINGS_PROPERTY_IRI_HTTP || *other == SIBLINGS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SiblingsPropertyIri> for &str {
	fn eq(&self, other: &SiblingsPropertyIri) -> bool {
		*self == SIBLINGS_PROPERTY_IRI_HTTP || *self == SIBLINGS_PROPERTY_IRI_HTTPS
	}
}
pub struct SiblingsPropertyIriOrLabel;
impl PartialEq<&str> for SiblingsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SiblingsPropertyIri || *other == SIBLINGS_PROPERTY_LABEL
	}
}
impl PartialEq<SiblingsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SiblingsPropertyIriOrLabel) -> bool {
		*self == SiblingsPropertyIri || *self == SIBLINGS_PROPERTY_LABEL
	}
}
