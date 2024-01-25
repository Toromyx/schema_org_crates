/// <https://schema.org/Nonprofit501c4>
pub const NONPROFIT_501_C_4_IRI_HTTP: &str = "http://schema.org/Nonprofit501c4";
/// <https://schema.org/Nonprofit501c4>
pub const NONPROFIT_501_C_4_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c4";
/// <https://schema.org/Nonprofit501c4>
pub const NONPROFIT_501_C_4_LABEL: &str = "Nonprofit501c4";
pub struct Nonprofit501C4Iri;
impl PartialEq<&str> for Nonprofit501C4Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_4_IRI_HTTP || *other == NONPROFIT_501_C_4_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C4Iri> for &str {
	fn eq(&self, other: &Nonprofit501C4Iri) -> bool {
		*self == NONPROFIT_501_C_4_IRI_HTTP || *self == NONPROFIT_501_C_4_IRI_HTTPS
	}
}
pub struct Nonprofit501C4IriOrLabel;
impl PartialEq<&str> for Nonprofit501C4IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C4Iri || *other == NONPROFIT_501_C_4_LABEL
	}
}
impl PartialEq<Nonprofit501C4IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C4IriOrLabel) -> bool {
		*self == Nonprofit501C4Iri || *self == NONPROFIT_501_C_4_LABEL
	}
}
