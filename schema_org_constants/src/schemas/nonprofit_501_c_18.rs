/// <https://schema.org/Nonprofit501c18>
pub const NONPROFIT_501_C_18_IRI_HTTP: &str = "http://schema.org/Nonprofit501c18";
/// <https://schema.org/Nonprofit501c18>
pub const NONPROFIT_501_C_18_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c18";
/// <https://schema.org/Nonprofit501c18>
pub const NONPROFIT_501_C_18_LABEL: &str = "Nonprofit501c18";
pub struct Nonprofit501C18Iri;
impl PartialEq<&str> for Nonprofit501C18Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_18_IRI_HTTP || *other == NONPROFIT_501_C_18_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C18Iri> for &str {
	fn eq(&self, other: &Nonprofit501C18Iri) -> bool {
		*self == NONPROFIT_501_C_18_IRI_HTTP || *self == NONPROFIT_501_C_18_IRI_HTTPS
	}
}
pub struct Nonprofit501C18IriOrLabel;
impl PartialEq<&str> for Nonprofit501C18IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C18Iri || *other == NONPROFIT_501_C_18_LABEL
	}
}
impl PartialEq<Nonprofit501C18IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C18IriOrLabel) -> bool {
		*self == Nonprofit501C18Iri || *self == NONPROFIT_501_C_18_LABEL
	}
}
