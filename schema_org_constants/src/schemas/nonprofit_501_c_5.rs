/// <https://schema.org/Nonprofit501c5>
pub const NONPROFIT_501_C_5_IRI_HTTP: &str = "http://schema.org/Nonprofit501c5";
/// <https://schema.org/Nonprofit501c5>
pub const NONPROFIT_501_C_5_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c5";
/// <https://schema.org/Nonprofit501c5>
pub const NONPROFIT_501_C_5_LABEL: &str = "Nonprofit501c5";
pub struct Nonprofit501C5Iri;
impl PartialEq<&str> for Nonprofit501C5Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_5_IRI_HTTP || *other == NONPROFIT_501_C_5_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C5Iri> for &str {
	fn eq(&self, other: &Nonprofit501C5Iri) -> bool {
		*self == NONPROFIT_501_C_5_IRI_HTTP || *self == NONPROFIT_501_C_5_IRI_HTTPS
	}
}
pub struct Nonprofit501C5IriOrLabel;
impl PartialEq<&str> for Nonprofit501C5IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C5Iri || *other == NONPROFIT_501_C_5_LABEL
	}
}
impl PartialEq<Nonprofit501C5IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C5IriOrLabel) -> bool {
		*self == Nonprofit501C5Iri || *self == NONPROFIT_501_C_5_LABEL
	}
}
