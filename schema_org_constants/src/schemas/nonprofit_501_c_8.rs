/// <https://schema.org/Nonprofit501c8>
pub const NONPROFIT_501_C_8_IRI_HTTP: &str = "http://schema.org/Nonprofit501c8";
/// <https://schema.org/Nonprofit501c8>
pub const NONPROFIT_501_C_8_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c8";
/// <https://schema.org/Nonprofit501c8>
pub const NONPROFIT_501_C_8_LABEL: &str = "Nonprofit501c8";
pub struct Nonprofit501C8Iri;
impl PartialEq<&str> for Nonprofit501C8Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_8_IRI_HTTP || *other == NONPROFIT_501_C_8_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C8Iri> for &str {
	fn eq(&self, other: &Nonprofit501C8Iri) -> bool {
		*self == NONPROFIT_501_C_8_IRI_HTTP || *self == NONPROFIT_501_C_8_IRI_HTTPS
	}
}
pub struct Nonprofit501C8IriOrLabel;
impl PartialEq<&str> for Nonprofit501C8IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C8Iri || *other == NONPROFIT_501_C_8_LABEL
	}
}
impl PartialEq<Nonprofit501C8IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C8IriOrLabel) -> bool {
		*self == Nonprofit501C8Iri || *self == NONPROFIT_501_C_8_LABEL
	}
}
