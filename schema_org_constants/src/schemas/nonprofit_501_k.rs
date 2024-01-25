/// <https://schema.org/Nonprofit501k>
pub const NONPROFIT_501_K_IRI_HTTP: &str = "http://schema.org/Nonprofit501k";
/// <https://schema.org/Nonprofit501k>
pub const NONPROFIT_501_K_IRI_HTTPS: &str = "https://schema.org/Nonprofit501k";
/// <https://schema.org/Nonprofit501k>
pub const NONPROFIT_501_K_LABEL: &str = "Nonprofit501k";
pub struct Nonprofit501KIri;
impl PartialEq<&str> for Nonprofit501KIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_501_K_IRI_HTTP || *other == NONPROFIT_501_K_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit501KIri> for &str {
	fn eq(&self, other: &Nonprofit501KIri) -> bool {
		*self == NONPROFIT_501_K_IRI_HTTP || *self == NONPROFIT_501_K_IRI_HTTPS
	}
}
pub struct Nonprofit501KIriOrLabel;
impl PartialEq<&str> for Nonprofit501KIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit501KIri || *other == NONPROFIT_501_K_LABEL
	}
}
impl PartialEq<Nonprofit501KIriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit501KIriOrLabel) -> bool {
		*self == Nonprofit501KIri || *self == NONPROFIT_501_K_LABEL
	}
}
