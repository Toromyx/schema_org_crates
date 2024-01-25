/// <https://schema.org/Nonprofit501c25>
pub const NONPROFIT_501_C_25_IRI_HTTP: &str = "http://schema.org/Nonprofit501c25";
/// <https://schema.org/Nonprofit501c25>
pub const NONPROFIT_501_C_25_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c25";
/// <https://schema.org/Nonprofit501c25>
pub const NONPROFIT_501_C_25_LABEL: &str = "Nonprofit501c25";
pub struct Nonprofit501C25Iri;
impl PartialEq<&str> for Nonprofit501C25Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_25_IRI_HTTP || *other == NONPROFIT_501_C_25_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C25Iri> for &str {
	fn eq(&self, other: &Nonprofit501C25Iri) -> bool {
		*self == NONPROFIT_501_C_25_IRI_HTTP || *self == NONPROFIT_501_C_25_IRI_HTTPS
	}
}
pub struct Nonprofit501C25IriOrLabel;
impl PartialEq<&str> for Nonprofit501C25IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C25Iri || *other == NONPROFIT_501_C_25_LABEL
	}
}
impl PartialEq<Nonprofit501C25IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C25IriOrLabel) -> bool {
		*self == Nonprofit501C25Iri || *self == NONPROFIT_501_C_25_LABEL
	}
}
