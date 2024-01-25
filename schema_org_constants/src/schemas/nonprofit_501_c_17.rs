/// <https://schema.org/Nonprofit501c17>
pub const NONPROFIT_501_C_17_IRI_HTTP: &str = "http://schema.org/Nonprofit501c17";
/// <https://schema.org/Nonprofit501c17>
pub const NONPROFIT_501_C_17_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c17";
/// <https://schema.org/Nonprofit501c17>
pub const NONPROFIT_501_C_17_LABEL: &str = "Nonprofit501c17";
pub struct Nonprofit501C17Iri;
impl PartialEq<&str> for Nonprofit501C17Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_17_IRI_HTTP || *other == NONPROFIT_501_C_17_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C17Iri> for &str {
	fn eq(&self, other: &Nonprofit501C17Iri) -> bool {
		*self == NONPROFIT_501_C_17_IRI_HTTP || *self == NONPROFIT_501_C_17_IRI_HTTPS
	}
}
pub struct Nonprofit501C17IriOrLabel;
impl PartialEq<&str> for Nonprofit501C17IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C17Iri || *other == NONPROFIT_501_C_17_LABEL
	}
}
impl PartialEq<Nonprofit501C17IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C17IriOrLabel) -> bool {
		*self == Nonprofit501C17Iri || *self == NONPROFIT_501_C_17_LABEL
	}
}
