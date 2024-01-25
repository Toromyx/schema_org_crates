/// <https://schema.org/Nonprofit501c15>
pub const NONPROFIT_501_C_15_IRI_HTTP: &str = "http://schema.org/Nonprofit501c15";
/// <https://schema.org/Nonprofit501c15>
pub const NONPROFIT_501_C_15_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c15";
/// <https://schema.org/Nonprofit501c15>
pub const NONPROFIT_501_C_15_LABEL: &str = "Nonprofit501c15";
pub struct Nonprofit501C15Iri;
impl PartialEq<&str> for Nonprofit501C15Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_15_IRI_HTTP || *other == NONPROFIT_501_C_15_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C15Iri> for &str {
	fn eq(&self, other: &Nonprofit501C15Iri) -> bool {
		*self == NONPROFIT_501_C_15_IRI_HTTP || *self == NONPROFIT_501_C_15_IRI_HTTPS
	}
}
pub struct Nonprofit501C15IriOrLabel;
impl PartialEq<&str> for Nonprofit501C15IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C15Iri || *other == NONPROFIT_501_C_15_LABEL
	}
}
impl PartialEq<Nonprofit501C15IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C15IriOrLabel) -> bool {
		*self == Nonprofit501C15Iri || *self == NONPROFIT_501_C_15_LABEL
	}
}
