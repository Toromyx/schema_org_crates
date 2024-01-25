/// <https://schema.org/Nonprofit501c27>
pub const NONPROFIT_501_C_27_IRI_HTTP: &str = "http://schema.org/Nonprofit501c27";
/// <https://schema.org/Nonprofit501c27>
pub const NONPROFIT_501_C_27_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c27";
/// <https://schema.org/Nonprofit501c27>
pub const NONPROFIT_501_C_27_LABEL: &str = "Nonprofit501c27";
pub struct Nonprofit501C27Iri;
impl PartialEq<&str> for Nonprofit501C27Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_27_IRI_HTTP || *other == NONPROFIT_501_C_27_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C27Iri> for &str {
	fn eq(&self, other: &Nonprofit501C27Iri) -> bool {
		*self == NONPROFIT_501_C_27_IRI_HTTP || *self == NONPROFIT_501_C_27_IRI_HTTPS
	}
}
pub struct Nonprofit501C27IriOrLabel;
impl PartialEq<&str> for Nonprofit501C27IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C27Iri || *other == NONPROFIT_501_C_27_LABEL
	}
}
impl PartialEq<Nonprofit501C27IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C27IriOrLabel) -> bool {
		*self == Nonprofit501C27Iri || *self == NONPROFIT_501_C_27_LABEL
	}
}
