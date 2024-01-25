/// <https://schema.org/followee>
pub const FOLLOWEE_PROPERTY_IRI_HTTP: &str = "http://schema.org/followee";
/// <https://schema.org/followee>
pub const FOLLOWEE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/followee";
/// <https://schema.org/followee>
pub const FOLLOWEE_PROPERTY_LABEL: &str = "followee";
pub struct FolloweePropertyIri;
impl PartialEq<&str> for FolloweePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOLLOWEE_PROPERTY_IRI_HTTP || *other == FOLLOWEE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FolloweePropertyIri> for &str {
	fn eq(&self, other: &FolloweePropertyIri) -> bool {
		*self == FOLLOWEE_PROPERTY_IRI_HTTP || *self == FOLLOWEE_PROPERTY_IRI_HTTPS
	}
}
pub struct FolloweePropertyIriOrLabel;
impl PartialEq<&str> for FolloweePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FolloweePropertyIri || *other == FOLLOWEE_PROPERTY_LABEL
	}
}
impl PartialEq<FolloweePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FolloweePropertyIriOrLabel) -> bool {
		*self == FolloweePropertyIri || *self == FOLLOWEE_PROPERTY_LABEL
	}
}
