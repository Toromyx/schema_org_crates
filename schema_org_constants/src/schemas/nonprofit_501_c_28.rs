/// <https://schema.org/Nonprofit501c28>
pub const NONPROFIT_501_C_28_IRI_HTTP: &str = "http://schema.org/Nonprofit501c28";
/// <https://schema.org/Nonprofit501c28>
pub const NONPROFIT_501_C_28_IRI_HTTPS: &str = "https://schema.org/Nonprofit501c28";
/// <https://schema.org/Nonprofit501c28>
pub const NONPROFIT_501_C_28_LABEL: &str = "Nonprofit501c28";
pub struct Nonprofit501C28Iri;
impl PartialEq<&str> for Nonprofit501C28Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_C_28_IRI_HTTP || *other == NONPROFIT_501_C_28_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501C28Iri> for &str {
	fn eq(&self, other: &Nonprofit501C28Iri) -> bool {
		*self == NONPROFIT_501_C_28_IRI_HTTP || *self == NONPROFIT_501_C_28_IRI_HTTPS
	}
}
pub struct Nonprofit501C28IriOrLabel;
impl PartialEq<&str> for Nonprofit501C28IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501C28Iri || *other == NONPROFIT_501_C_28_LABEL
	}
}
impl PartialEq<Nonprofit501C28IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501C28IriOrLabel) -> bool {
		*self == Nonprofit501C28Iri || *self == NONPROFIT_501_C_28_LABEL
	}
}
