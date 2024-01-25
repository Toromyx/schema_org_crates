/// <https://schema.org/Nonprofit501c16>
pub const NONPROFIT_501_C_16_IRI_HTTP: &str = "http://schema.org/Nonprofit501c16";
/// <https://schema.org/Nonprofit501c16>
pub const NONPROFIT_501_C_16_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c16";
/// <https://schema.org/Nonprofit501c16>
pub const NONPROFIT_501_C_16_LABEL: &str = "Nonprofit501c16";
pub struct Nonprofit501C16Iri;
impl PartialEq<&str> for Nonprofit501C16Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_16_IRI_HTTP || *other == NONPROFIT_501_C_16_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C16Iri> for &str {
	fn eq(&self, other: &Nonprofit501C16Iri) -> bool {
		*self == NONPROFIT_501_C_16_IRI_HTTP || *self == NONPROFIT_501_C_16_IRI_HTTPS
	}
}
pub struct Nonprofit501C16IriOrLabel;
impl PartialEq<&str> for Nonprofit501C16IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C16Iri || *other == NONPROFIT_501_C_16_LABEL
	}
}
impl PartialEq<Nonprofit501C16IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C16IriOrLabel) -> bool {
		*self == Nonprofit501C16Iri || *self == NONPROFIT_501_C_16_LABEL
	}
}
