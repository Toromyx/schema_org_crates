/// <https://schema.org/releaseOf>
pub const RELEASE_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/releaseOf";
/// <https://schema.org/releaseOf>
pub const RELEASE_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/releaseOf";
/// <https://schema.org/releaseOf>
pub const RELEASE_OF_PROPERTY_LABEL: &str = "releaseOf";
pub struct ReleaseOfPropertyIri;
impl PartialEq<&str> for ReleaseOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELEASE_OF_PROPERTY_IRI_HTTP || *other == RELEASE_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReleaseOfPropertyIri> for &str {
	fn eq(&self, other: &ReleaseOfPropertyIri) -> bool {
		*self == RELEASE_OF_PROPERTY_IRI_HTTP || *self == RELEASE_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct ReleaseOfPropertyIriOrLabel;
impl PartialEq<&str> for ReleaseOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReleaseOfPropertyIri || *other == RELEASE_OF_PROPERTY_LABEL
	}
}
impl PartialEq<ReleaseOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReleaseOfPropertyIriOrLabel) -> bool {
		*self == ReleaseOfPropertyIri || *self == RELEASE_OF_PROPERTY_LABEL
	}
}
