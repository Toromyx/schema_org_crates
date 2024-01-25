/// <https://schema.org/Podiatric>
pub const PODIATRIC_IRI_HTTP: &str = "http://schema.org/Podiatric";
/// <https://schema.org/Podiatric>
pub const PODIATRIC_IRI_HTTPS: &str = "https://schema.org/Podiatric";
/// <https://schema.org/Podiatric>
pub const PODIATRIC_LABEL: &str = "Podiatric";
pub struct PodiatricIri;
impl PartialEq<&str> for PodiatricIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PODIATRIC_IRI_HTTP || *other == PODIATRIC_IRI_HTTPS
	}
}
impl PartialEq<PodiatricIri> for &str {
	fn eq(&self, other: &PodiatricIri) -> bool {
		*self == PODIATRIC_IRI_HTTP || *self == PODIATRIC_IRI_HTTPS
	}
}
pub struct PodiatricIriOrLabel;
impl PartialEq<&str> for PodiatricIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PodiatricIri || *other == PODIATRIC_LABEL
	}
}
impl PartialEq<PodiatricIriOrLabel> for &str {
	fn eq(&self, other: &PodiatricIriOrLabel) -> bool {
		*self == PodiatricIri || *self == PODIATRIC_LABEL
	}
}
