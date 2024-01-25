/// <https://schema.org/FollowAction>
pub const FOLLOW_ACTION_IRI_HTTP: &str = "http://schema.org/FollowAction";
/// <https://schema.org/FollowAction>
pub const FOLLOW_ACTION_IRI_HTTPS: &str = "https://schema.org/FollowAction";
/// <https://schema.org/FollowAction>
pub const FOLLOW_ACTION_LABEL: &str = "FollowAction";
pub struct FollowActionIri;
impl PartialEq<&str> for FollowActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOLLOW_ACTION_IRI_HTTP || *other == FOLLOW_ACTION_IRI_HTTPS
	}
}
impl PartialEq<FollowActionIri> for &str {
	fn eq(&self, other: &FollowActionIri) -> bool {
		*self == FOLLOW_ACTION_IRI_HTTP || *self == FOLLOW_ACTION_IRI_HTTPS
	}
}
pub struct FollowActionIriOrLabel;
impl PartialEq<&str> for FollowActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FollowActionIri || *other == FOLLOW_ACTION_LABEL
	}
}
impl PartialEq<FollowActionIriOrLabel> for &str {
	fn eq(&self, other: &FollowActionIriOrLabel) -> bool {
		*self == FollowActionIri || *self == FOLLOW_ACTION_LABEL
	}
}
