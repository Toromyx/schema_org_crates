/// <https://schema.org/Nonprofit501c26>
pub const NONPROFIT_501_C_26_IRI_HTTP: &str = "http://schema.org/Nonprofit501c26";
/// <https://schema.org/Nonprofit501c26>
pub const NONPROFIT_501_C_26_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c26";
/// <https://schema.org/Nonprofit501c26>
pub const NONPROFIT_501_C_26_LABEL: &str = "Nonprofit501c26";
pub struct Nonprofit501C26Iri;
impl PartialEq<&str> for Nonprofit501C26Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_26_IRI_HTTP || *other == NONPROFIT_501_C_26_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C26Iri> for &str {
	fn eq(&self, other: &Nonprofit501C26Iri) -> bool {
		*self == NONPROFIT_501_C_26_IRI_HTTP || *self == NONPROFIT_501_C_26_IRI_HTTPS
	}
}
pub struct Nonprofit501C26IriOrLabel;
impl PartialEq<&str> for Nonprofit501C26IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C26Iri || *other == NONPROFIT_501_C_26_LABEL
	}
}
impl PartialEq<Nonprofit501C26IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C26IriOrLabel) -> bool {
		*self == Nonprofit501C26Iri || *self == NONPROFIT_501_C_26_LABEL
	}
}
