/// <https://schema.org/Nonprofit501c2>
pub const NONPROFIT_501_C_2_IRI_HTTP: &str = "http://schema.org/Nonprofit501c2";
/// <https://schema.org/Nonprofit501c2>
pub const NONPROFIT_501_C_2_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c2";
/// <https://schema.org/Nonprofit501c2>
pub const NONPROFIT_501_C_2_LABEL: &str = "Nonprofit501c2";
pub struct Nonprofit501C2Iri;
impl PartialEq<&str> for Nonprofit501C2Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_2_IRI_HTTP || *other == NONPROFIT_501_C_2_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C2Iri> for &str {
	fn eq(&self, other: &Nonprofit501C2Iri) -> bool {
		*self == NONPROFIT_501_C_2_IRI_HTTP || *self == NONPROFIT_501_C_2_IRI_HTTPS
	}
}
pub struct Nonprofit501C2IriOrLabel;
impl PartialEq<&str> for Nonprofit501C2IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C2Iri || *other == NONPROFIT_501_C_2_LABEL
	}
}
impl PartialEq<Nonprofit501C2IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C2IriOrLabel) -> bool {
		*self == Nonprofit501C2Iri || *self == NONPROFIT_501_C_2_LABEL
	}
}
