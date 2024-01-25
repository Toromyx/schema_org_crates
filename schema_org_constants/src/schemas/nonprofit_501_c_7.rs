/// <https://schema.org/Nonprofit501c7>
pub const NONPROFIT_501_C_7_IRI_HTTP: &str = "http://schema.org/Nonprofit501c7";
/// <https://schema.org/Nonprofit501c7>
pub const NONPROFIT_501_C_7_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c7";
/// <https://schema.org/Nonprofit501c7>
pub const NONPROFIT_501_C_7_LABEL: &str = "Nonprofit501c7";
pub struct Nonprofit501C7Iri;
impl PartialEq<&str> for Nonprofit501C7Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_7_IRI_HTTP || *other == NONPROFIT_501_C_7_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C7Iri> for &str {
	fn eq(&self, other: &Nonprofit501C7Iri) -> bool {
		*self == NONPROFIT_501_C_7_IRI_HTTP || *self == NONPROFIT_501_C_7_IRI_HTTPS
	}
}
pub struct Nonprofit501C7IriOrLabel;
impl PartialEq<&str> for Nonprofit501C7IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C7Iri || *other == NONPROFIT_501_C_7_LABEL
	}
}
impl PartialEq<Nonprofit501C7IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C7IriOrLabel) -> bool {
		*self == Nonprofit501C7Iri || *self == NONPROFIT_501_C_7_LABEL
	}
}
