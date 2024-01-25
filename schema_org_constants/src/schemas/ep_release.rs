/// <https://schema.org/EPRelease>
pub const EP_RELEASE_IRI_HTTP: &str = "http://schema.org/EPRelease";
/// <https://schema.org/EPRelease>
pub const EP_RELEASE_IRI_HTTPS: &str = "https://schema.org/EPRelease";
/// <https://schema.org/EPRelease>
pub const EP_RELEASE_LABEL: &str = "EPRelease";
pub struct EpReleaseIri;
impl PartialEq<&str> for EpReleaseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EP_RELEASE_IRI_HTTP || *other == EP_RELEASE_IRI_HTTPS
	}
}
impl PartialEq<EpReleaseIri> for &str {
	fn eq(&self, other: &EpReleaseIri) -> bool {
		*self == EP_RELEASE_IRI_HTTP || *self == EP_RELEASE_IRI_HTTPS
	}
}
pub struct EpReleaseIriOrLabel;
impl PartialEq<&str> for EpReleaseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EpReleaseIri || *other == EP_RELEASE_LABEL
	}
}
impl PartialEq<EpReleaseIriOrLabel> for &str {
	fn eq(&self, other: &EpReleaseIriOrLabel) -> bool {
		*self == EpReleaseIri || *self == EP_RELEASE_LABEL
	}
}
