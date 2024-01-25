/// <https://schema.org/Intangible>
pub const INTANGIBLE_IRI_HTTP: &str = "http://schema.org/Intangible";
/// <https://schema.org/Intangible>
pub const INTANGIBLE_IRI_HTTPS: &str = "https://schema.org/Intangible";
/// <https://schema.org/Intangible>
pub const INTANGIBLE_LABEL: &str = "Intangible";
pub struct IntangibleIri;
impl PartialEq<&str> for IntangibleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTANGIBLE_IRI_HTTP || *other == INTANGIBLE_IRI_HTTPS
	}
}
impl PartialEq<IntangibleIri> for &str {
	fn eq(&self, other: &IntangibleIri) -> bool {
		*self == INTANGIBLE_IRI_HTTP || *self == INTANGIBLE_IRI_HTTPS
	}
}
pub struct IntangibleIriOrLabel;
impl PartialEq<&str> for IntangibleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IntangibleIri || *other == INTANGIBLE_LABEL
	}
}
impl PartialEq<IntangibleIriOrLabel> for &str {
	fn eq(&self, other: &IntangibleIriOrLabel) -> bool {
		*self == IntangibleIri || *self == INTANGIBLE_LABEL
	}
}
