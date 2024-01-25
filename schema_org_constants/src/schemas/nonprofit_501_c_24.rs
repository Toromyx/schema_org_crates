/// <https://schema.org/Nonprofit501c24>
pub const NONPROFIT_501_C_24_IRI_HTTP: &str = "http://schema.org/Nonprofit501c24";
/// <https://schema.org/Nonprofit501c24>
pub const NONPROFIT_501_C_24_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c24";
/// <https://schema.org/Nonprofit501c24>
pub const NONPROFIT_501_C_24_LABEL: &str = "Nonprofit501c24";
pub struct Nonprofit501C24Iri;
impl PartialEq<&str> for Nonprofit501C24Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_24_IRI_HTTP || *other == NONPROFIT_501_C_24_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C24Iri> for &str {
	fn eq(&self, other: &Nonprofit501C24Iri) -> bool {
		*self == NONPROFIT_501_C_24_IRI_HTTP || *self == NONPROFIT_501_C_24_IRI_HTTPS
	}
}
pub struct Nonprofit501C24IriOrLabel;
impl PartialEq<&str> for Nonprofit501C24IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C24Iri || *other == NONPROFIT_501_C_24_LABEL
	}
}
impl PartialEq<Nonprofit501C24IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C24IriOrLabel) -> bool {
		*self == Nonprofit501C24Iri || *self == NONPROFIT_501_C_24_LABEL
	}
}
