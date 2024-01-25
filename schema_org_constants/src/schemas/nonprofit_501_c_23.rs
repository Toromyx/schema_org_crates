/// <https://schema.org/Nonprofit501c23>
pub const NONPROFIT_501_C_23_IRI_HTTP: &str = "http://schema.org/Nonprofit501c23";
/// <https://schema.org/Nonprofit501c23>
pub const NONPROFIT_501_C_23_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c23";
/// <https://schema.org/Nonprofit501c23>
pub const NONPROFIT_501_C_23_LABEL: &str = "Nonprofit501c23";
pub struct Nonprofit501C23Iri;
impl PartialEq<&str> for Nonprofit501C23Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_23_IRI_HTTP || *other == NONPROFIT_501_C_23_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C23Iri> for &str {
	fn eq(&self, other: &Nonprofit501C23Iri) -> bool {
		*self == NONPROFIT_501_C_23_IRI_HTTP || *self == NONPROFIT_501_C_23_IRI_HTTPS
	}
}
pub struct Nonprofit501C23IriOrLabel;
impl PartialEq<&str> for Nonprofit501C23IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C23Iri || *other == NONPROFIT_501_C_23_LABEL
	}
}
impl PartialEq<Nonprofit501C23IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C23IriOrLabel) -> bool {
		*self == Nonprofit501C23Iri || *self == NONPROFIT_501_C_23_LABEL
	}
}
