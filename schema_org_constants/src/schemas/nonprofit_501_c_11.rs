/// <https://schema.org/Nonprofit501c11>
pub const NONPROFIT_501_C_11_IRI_HTTP: &str = "http://schema.org/Nonprofit501c11";
/// <https://schema.org/Nonprofit501c11>
pub const NONPROFIT_501_C_11_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c11";
/// <https://schema.org/Nonprofit501c11>
pub const NONPROFIT_501_C_11_LABEL: &str = "Nonprofit501c11";
pub struct Nonprofit501C11Iri;
impl PartialEq<&str> for Nonprofit501C11Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_11_IRI_HTTP || *other == NONPROFIT_501_C_11_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C11Iri> for &str {
	fn eq(&self, other: &Nonprofit501C11Iri) -> bool {
		*self == NONPROFIT_501_C_11_IRI_HTTP || *self == NONPROFIT_501_C_11_IRI_HTTPS
	}
}
pub struct Nonprofit501C11IriOrLabel;
impl PartialEq<&str> for Nonprofit501C11IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C11Iri || *other == NONPROFIT_501_C_11_LABEL
	}
}
impl PartialEq<Nonprofit501C11IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C11IriOrLabel) -> bool {
		*self == Nonprofit501C11Iri || *self == NONPROFIT_501_C_11_LABEL
	}
}
