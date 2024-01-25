/// <https://schema.org/Nonprofit501c9>
pub const NONPROFIT_501_C_9_IRI_HTTP: &str = "http://schema.org/Nonprofit501c9";
/// <https://schema.org/Nonprofit501c9>
pub const NONPROFIT_501_C_9_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c9";
/// <https://schema.org/Nonprofit501c9>
pub const NONPROFIT_501_C_9_LABEL: &str = "Nonprofit501c9";
pub struct Nonprofit501C9Iri;
impl PartialEq<&str> for Nonprofit501C9Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_9_IRI_HTTP || *other == NONPROFIT_501_C_9_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C9Iri> for &str {
	fn eq(&self, other: &Nonprofit501C9Iri) -> bool {
		*self == NONPROFIT_501_C_9_IRI_HTTP || *self == NONPROFIT_501_C_9_IRI_HTTPS
	}
}
pub struct Nonprofit501C9IriOrLabel;
impl PartialEq<&str> for Nonprofit501C9IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C9Iri || *other == NONPROFIT_501_C_9_LABEL
	}
}
impl PartialEq<Nonprofit501C9IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C9IriOrLabel) -> bool {
		*self == Nonprofit501C9Iri || *self == NONPROFIT_501_C_9_LABEL
	}
}
