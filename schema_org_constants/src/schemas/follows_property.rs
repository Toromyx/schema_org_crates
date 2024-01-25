/// <https://schema.org/follows>
pub const FOLLOWS_PROPERTY_IRI_HTTP: &str = "http://schema.org/follows";
/// <https://schema.org/follows>
pub const FOLLOWS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/follows";
/// <https://schema.org/follows>
pub const FOLLOWS_PROPERTY_LABEL: &str = "follows";
pub struct FollowsPropertyIri;
impl PartialEq<&str> for FollowsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOLLOWS_PROPERTY_IRI_HTTP || *other == FOLLOWS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FollowsPropertyIri> for &str {
	fn eq(&self, other: &FollowsPropertyIri) -> bool {
		*self == FOLLOWS_PROPERTY_IRI_HTTP || *self == FOLLOWS_PROPERTY_IRI_HTTPS
	}
}
pub struct FollowsPropertyIriOrLabel;
impl PartialEq<&str> for FollowsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FollowsPropertyIri || *other == FOLLOWS_PROPERTY_LABEL
	}
}
impl PartialEq<FollowsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FollowsPropertyIriOrLabel) -> bool {
		*self == FollowsPropertyIri || *self == FOLLOWS_PROPERTY_LABEL
	}
}
