/// <https://schema.org/Nonprofit501c1>
pub const NONPROFIT_501_C_1_IRI_HTTP: &str = "http://schema.org/Nonprofit501c1";
/// <https://schema.org/Nonprofit501c1>
pub const NONPROFIT_501_C_1_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c1";
/// <https://schema.org/Nonprofit501c1>
pub const NONPROFIT_501_C_1_LABEL: &str = "Nonprofit501c1";
pub struct Nonprofit501C1Iri;
impl PartialEq<&str> for Nonprofit501C1Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_1_IRI_HTTP || *other == NONPROFIT_501_C_1_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C1Iri> for &str {
	fn eq(&self, other: &Nonprofit501C1Iri) -> bool {
		*self == NONPROFIT_501_C_1_IRI_HTTP || *self == NONPROFIT_501_C_1_IRI_HTTPS
	}
}
pub struct Nonprofit501C1IriOrLabel;
impl PartialEq<&str> for Nonprofit501C1IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C1Iri || *other == NONPROFIT_501_C_1_LABEL
	}
}
impl PartialEq<Nonprofit501C1IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C1IriOrLabel) -> bool {
		*self == Nonprofit501C1Iri || *self == NONPROFIT_501_C_1_LABEL
	}
}
