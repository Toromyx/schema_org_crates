/// <https://schema.org/Nonprofit501c14>
pub const NONPROFIT_501_C_14_IRI_HTTP: &str = "http://schema.org/Nonprofit501c14";
/// <https://schema.org/Nonprofit501c14>
pub const NONPROFIT_501_C_14_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c14";
/// <https://schema.org/Nonprofit501c14>
pub const NONPROFIT_501_C_14_LABEL: &str = "Nonprofit501c14";
pub struct Nonprofit501C14Iri;
impl PartialEq<&str> for Nonprofit501C14Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_14_IRI_HTTP || *other == NONPROFIT_501_C_14_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C14Iri> for &str {
	fn eq(&self, other: &Nonprofit501C14Iri) -> bool {
		*self == NONPROFIT_501_C_14_IRI_HTTP || *self == NONPROFIT_501_C_14_IRI_HTTPS
	}
}
pub struct Nonprofit501C14IriOrLabel;
impl PartialEq<&str> for Nonprofit501C14IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C14Iri || *other == NONPROFIT_501_C_14_LABEL
	}
}
impl PartialEq<Nonprofit501C14IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C14IriOrLabel) -> bool {
		*self == Nonprofit501C14Iri || *self == NONPROFIT_501_C_14_LABEL
	}
}
