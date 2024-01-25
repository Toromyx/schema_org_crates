/// <https://schema.org/Nonprofit501c22>
pub const NONPROFIT_501_C_22_IRI_HTTP: &str = "http://schema.org/Nonprofit501c22";
/// <https://schema.org/Nonprofit501c22>
pub const NONPROFIT_501_C_22_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c22";
/// <https://schema.org/Nonprofit501c22>
pub const NONPROFIT_501_C_22_LABEL: &str = "Nonprofit501c22";
pub struct Nonprofit501C22Iri;
impl PartialEq<&str> for Nonprofit501C22Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_22_IRI_HTTP || *other == NONPROFIT_501_C_22_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C22Iri> for &str {
	fn eq(&self, other: &Nonprofit501C22Iri) -> bool {
		*self == NONPROFIT_501_C_22_IRI_HTTP || *self == NONPROFIT_501_C_22_IRI_HTTPS
	}
}
pub struct Nonprofit501C22IriOrLabel;
impl PartialEq<&str> for Nonprofit501C22IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C22Iri || *other == NONPROFIT_501_C_22_LABEL
	}
}
impl PartialEq<Nonprofit501C22IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C22IriOrLabel) -> bool {
		*self == Nonprofit501C22Iri || *self == NONPROFIT_501_C_22_LABEL
	}
}
