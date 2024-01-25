/// <https://schema.org/Nonprofit501c6>
pub const NONPROFIT_501_C_6_IRI_HTTP: &str = "http://schema.org/Nonprofit501c6";
/// <https://schema.org/Nonprofit501c6>
pub const NONPROFIT_501_C_6_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c6";
/// <https://schema.org/Nonprofit501c6>
pub const NONPROFIT_501_C_6_LABEL: &str = "Nonprofit501c6";
pub struct Nonprofit501C6Iri;
impl PartialEq<&str> for Nonprofit501C6Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_6_IRI_HTTP || *other == NONPROFIT_501_C_6_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C6Iri> for &str {
	fn eq(&self, other: &Nonprofit501C6Iri) -> bool {
		*self == NONPROFIT_501_C_6_IRI_HTTP || *self == NONPROFIT_501_C_6_IRI_HTTPS
	}
}
pub struct Nonprofit501C6IriOrLabel;
impl PartialEq<&str> for Nonprofit501C6IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C6Iri || *other == NONPROFIT_501_C_6_LABEL
	}
}
impl PartialEq<Nonprofit501C6IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C6IriOrLabel) -> bool {
		*self == Nonprofit501C6Iri || *self == NONPROFIT_501_C_6_LABEL
	}
}
