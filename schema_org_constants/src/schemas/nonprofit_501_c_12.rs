/// <https://schema.org/Nonprofit501c12>
pub const NONPROFIT_501_C_12_IRI_HTTP: &str = "http://schema.org/Nonprofit501c12";
/// <https://schema.org/Nonprofit501c12>
pub const NONPROFIT_501_C_12_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c12";
/// <https://schema.org/Nonprofit501c12>
pub const NONPROFIT_501_C_12_LABEL: &str = "Nonprofit501c12";
pub struct Nonprofit501C12Iri;
impl PartialEq<&str> for Nonprofit501C12Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_12_IRI_HTTP || *other == NONPROFIT_501_C_12_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C12Iri> for &str {
	fn eq(&self, other: &Nonprofit501C12Iri) -> bool {
		*self == NONPROFIT_501_C_12_IRI_HTTP || *self == NONPROFIT_501_C_12_IRI_HTTPS
	}
}
pub struct Nonprofit501C12IriOrLabel;
impl PartialEq<&str> for Nonprofit501C12IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C12Iri || *other == NONPROFIT_501_C_12_LABEL
	}
}
impl PartialEq<Nonprofit501C12IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C12IriOrLabel) -> bool {
		*self == Nonprofit501C12Iri || *self == NONPROFIT_501_C_12_LABEL
	}
}
