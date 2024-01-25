/// <https://schema.org/Nonprofit501c20>
pub const NONPROFIT_501_C_20_IRI_HTTP: &str = "http://schema.org/Nonprofit501c20";
/// <https://schema.org/Nonprofit501c20>
pub const NONPROFIT_501_C_20_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c20";
/// <https://schema.org/Nonprofit501c20>
pub const NONPROFIT_501_C_20_LABEL: &str = "Nonprofit501c20";
pub struct Nonprofit501C20Iri;
impl PartialEq<&str> for Nonprofit501C20Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_20_IRI_HTTP || *other == NONPROFIT_501_C_20_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C20Iri> for &str {
	fn eq(&self, other: &Nonprofit501C20Iri) -> bool {
		*self == NONPROFIT_501_C_20_IRI_HTTP || *self == NONPROFIT_501_C_20_IRI_HTTPS
	}
}
pub struct Nonprofit501C20IriOrLabel;
impl PartialEq<&str> for Nonprofit501C20IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C20Iri || *other == NONPROFIT_501_C_20_LABEL
	}
}
impl PartialEq<Nonprofit501C20IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C20IriOrLabel) -> bool {
		*self == Nonprofit501C20Iri || *self == NONPROFIT_501_C_20_LABEL
	}
}
