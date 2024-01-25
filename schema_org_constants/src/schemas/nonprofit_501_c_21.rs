/// <https://schema.org/Nonprofit501c21>
pub const NONPROFIT_501_C_21_IRI_HTTP: &str = "http://schema.org/Nonprofit501c21";
/// <https://schema.org/Nonprofit501c21>
pub const NONPROFIT_501_C_21_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c21";
/// <https://schema.org/Nonprofit501c21>
pub const NONPROFIT_501_C_21_LABEL: &str = "Nonprofit501c21";
pub struct Nonprofit501C21Iri;
impl PartialEq<&str> for Nonprofit501C21Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_21_IRI_HTTP || *other == NONPROFIT_501_C_21_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C21Iri> for &str {
	fn eq(&self, other: &Nonprofit501C21Iri) -> bool {
		*self == NONPROFIT_501_C_21_IRI_HTTP || *self == NONPROFIT_501_C_21_IRI_HTTPS
	}
}
pub struct Nonprofit501C21IriOrLabel;
impl PartialEq<&str> for Nonprofit501C21IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C21Iri || *other == NONPROFIT_501_C_21_LABEL
	}
}
impl PartialEq<Nonprofit501C21IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C21IriOrLabel) -> bool {
		*self == Nonprofit501C21Iri || *self == NONPROFIT_501_C_21_LABEL
	}
}
