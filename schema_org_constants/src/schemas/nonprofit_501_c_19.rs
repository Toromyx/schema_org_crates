/// <https://schema.org/Nonprofit501c19>
pub const NONPROFIT_501_C_19_IRI_HTTP: &str = "http://schema.org/Nonprofit501c19";
/// <https://schema.org/Nonprofit501c19>
pub const NONPROFIT_501_C_19_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c19";
/// <https://schema.org/Nonprofit501c19>
pub const NONPROFIT_501_C_19_LABEL: &str = "Nonprofit501c19";
pub struct Nonprofit501C19Iri;
impl PartialEq<&str> for Nonprofit501C19Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_19_IRI_HTTP || *other == NONPROFIT_501_C_19_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C19Iri> for &str {
	fn eq(&self, other: &Nonprofit501C19Iri) -> bool {
		*self == NONPROFIT_501_C_19_IRI_HTTP || *self == NONPROFIT_501_C_19_IRI_HTTPS
	}
}
pub struct Nonprofit501C19IriOrLabel;
impl PartialEq<&str> for Nonprofit501C19IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C19Iri || *other == NONPROFIT_501_C_19_LABEL
	}
}
impl PartialEq<Nonprofit501C19IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C19IriOrLabel) -> bool {
		*self == Nonprofit501C19Iri || *self == NONPROFIT_501_C_19_LABEL
	}
}
