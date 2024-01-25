/// <https://schema.org/Nonprofit501c13>
pub const NONPROFIT_501_C_13_IRI_HTTP: &str = "http://schema.org/Nonprofit501c13";
/// <https://schema.org/Nonprofit501c13>
pub const NONPROFIT_501_C_13_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c13";
/// <https://schema.org/Nonprofit501c13>
pub const NONPROFIT_501_C_13_LABEL: &str = "Nonprofit501c13";
pub struct Nonprofit501C13Iri;
impl PartialEq<&str> for Nonprofit501C13Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_13_IRI_HTTP || *other == NONPROFIT_501_C_13_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C13Iri> for &str {
	fn eq(&self, other: &Nonprofit501C13Iri) -> bool {
		*self == NONPROFIT_501_C_13_IRI_HTTP || *self == NONPROFIT_501_C_13_IRI_HTTPS
	}
}
pub struct Nonprofit501C13IriOrLabel;
impl PartialEq<&str> for Nonprofit501C13IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C13Iri || *other == NONPROFIT_501_C_13_LABEL
	}
}
impl PartialEq<Nonprofit501C13IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C13IriOrLabel) -> bool {
		*self == Nonprofit501C13Iri || *self == NONPROFIT_501_C_13_LABEL
	}
}
