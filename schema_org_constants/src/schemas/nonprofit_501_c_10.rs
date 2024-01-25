/// <https://schema.org/Nonprofit501c10>
pub const NONPROFIT_501_C_10_IRI_HTTP: &str = "http://schema.org/Nonprofit501c10";
/// <https://schema.org/Nonprofit501c10>
pub const NONPROFIT_501_C_10_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c10";
/// <https://schema.org/Nonprofit501c10>
pub const NONPROFIT_501_C_10_LABEL: &str = "Nonprofit501c10";
pub struct Nonprofit501C10Iri;
impl PartialEq<&str> for Nonprofit501C10Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_10_IRI_HTTP || *other == NONPROFIT_501_C_10_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C10Iri> for &str {
	fn eq(&self, other: &Nonprofit501C10Iri) -> bool {
		*self == NONPROFIT_501_C_10_IRI_HTTP || *self == NONPROFIT_501_C_10_IRI_HTTPS
	}
}
pub struct Nonprofit501C10IriOrLabel;
impl PartialEq<&str> for Nonprofit501C10IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C10Iri || *other == NONPROFIT_501_C_10_LABEL
	}
}
impl PartialEq<Nonprofit501C10IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C10IriOrLabel) -> bool {
		*self == Nonprofit501C10Iri || *self == NONPROFIT_501_C_10_LABEL
	}
}
