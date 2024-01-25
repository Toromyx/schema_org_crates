/// <https://schema.org/Nonprofit501c3>
pub const NONPROFIT_501_C_3_IRI_HTTP: &str = "http://schema.org/Nonprofit501c3";
/// <https://schema.org/Nonprofit501c3>
pub const NONPROFIT_501_C_3_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c3";
/// <https://schema.org/Nonprofit501c3>
pub const NONPROFIT_501_C_3_LABEL: &str = "Nonprofit501c3";
pub struct Nonprofit501C3Iri;
impl PartialEq<&str> for Nonprofit501C3Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_3_IRI_HTTP || *other == NONPROFIT_501_C_3_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C3Iri> for &str {
	fn eq(&self, other: &Nonprofit501C3Iri) -> bool {
		*self == NONPROFIT_501_C_3_IRI_HTTP || *self == NONPROFIT_501_C_3_IRI_HTTPS
	}
}
pub struct Nonprofit501C3IriOrLabel;
impl PartialEq<&str> for Nonprofit501C3IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C3Iri || *other == NONPROFIT_501_C_3_LABEL
	}
}
impl PartialEq<Nonprofit501C3IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C3IriOrLabel) -> bool {
		*self == Nonprofit501C3Iri || *self == NONPROFIT_501_C_3_LABEL
	}
}
