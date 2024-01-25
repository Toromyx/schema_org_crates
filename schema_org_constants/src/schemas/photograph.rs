/// <https://schema.org/Photograph>
pub const PHOTOGRAPH_IRI_HTTP: &str = "http://schema.org/Photograph";
/// <https://schema.org/Photograph>
pub const PHOTOGRAPH_IRI_HTTPS: &str = "https://schema.org/Photograph";
/// <https://schema.org/Photograph>
pub const PHOTOGRAPH_LABEL: &str = "Photograph";
pub struct PhotographIri;
impl PartialEq<&str> for PhotographIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHOTOGRAPH_IRI_HTTP || *other == PHOTOGRAPH_IRI_HTTPS
	}
}
impl PartialEq<PhotographIri> for &str {
	fn eq(&self, other: &PhotographIri) -> bool {
		*self == PHOTOGRAPH_IRI_HTTP || *self == PHOTOGRAPH_IRI_HTTPS
	}
}
pub struct PhotographIriOrLabel;
impl PartialEq<&str> for PhotographIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhotographIri || *other == PHOTOGRAPH_LABEL
	}
}
impl PartialEq<PhotographIriOrLabel> for &str {
	fn eq(&self, other: &PhotographIriOrLabel) -> bool {
		*self == PhotographIri || *self == PHOTOGRAPH_LABEL
	}
}
