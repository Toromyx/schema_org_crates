/// <https://schema.org/SingleRelease>
pub const SINGLE_RELEASE_IRI_HTTP: &str = "http://schema.org/SingleRelease";
/// <https://schema.org/SingleRelease>
pub const SINGLE_RELEASE_IRI_HTTPS: &str = "https://schema.org/SingleRelease";
/// <https://schema.org/SingleRelease>
pub const SINGLE_RELEASE_LABEL: &str = "SingleRelease";
pub struct SingleReleaseIri;
impl PartialEq<&str> for SingleReleaseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SINGLE_RELEASE_IRI_HTTP || *other == SINGLE_RELEASE_IRI_HTTPS
	}
}
impl PartialEq<SingleReleaseIri> for &str {
	fn eq(&self, other: &SingleReleaseIri) -> bool {
		*self == SINGLE_RELEASE_IRI_HTTP || *self == SINGLE_RELEASE_IRI_HTTPS
	}
}
pub struct SingleReleaseIriOrLabel;
impl PartialEq<&str> for SingleReleaseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SingleReleaseIri || *other == SINGLE_RELEASE_LABEL
	}
}
impl PartialEq<SingleReleaseIriOrLabel> for &str {
	fn eq(&self, other: &SingleReleaseIriOrLabel) -> bool {
		*self == SingleReleaseIri || *self == SINGLE_RELEASE_LABEL
	}
}
