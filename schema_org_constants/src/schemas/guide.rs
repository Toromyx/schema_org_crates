/// <https://schema.org/Guide>
pub const GUIDE_IRI_HTTP: &str = "http://schema.org/Guide";
/// <https://schema.org/Guide>
pub const GUIDE_IRI_HTTPS: &str = "https://schema.org/Guide";
/// <https://schema.org/Guide>
pub const GUIDE_LABEL: &str = "Guide";
pub struct GuideIri;
impl PartialEq<&str> for GuideIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GUIDE_IRI_HTTP || *other == GUIDE_IRI_HTTPS
	}
}
impl PartialEq<GuideIri> for &str {
	fn eq(&self, other: &GuideIri) -> bool {
		*self == GUIDE_IRI_HTTP || *self == GUIDE_IRI_HTTPS
	}
}
pub struct GuideIriOrLabel;
impl PartialEq<&str> for GuideIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GuideIri || *other == GUIDE_LABEL
	}
}
impl PartialEq<GuideIriOrLabel> for &str {
	fn eq(&self, other: &GuideIriOrLabel) -> bool {
		*self == GuideIri || *self == GUIDE_LABEL
	}
}
