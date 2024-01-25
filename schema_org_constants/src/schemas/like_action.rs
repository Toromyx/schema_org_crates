/// <https://schema.org/LikeAction>
pub const LIKE_ACTION_IRI_HTTP: &str = "http://schema.org/LikeAction";
/// <https://schema.org/LikeAction>
pub const LIKE_ACTION_IRI_HTTPS: &str = "https://schema.org/LikeAction";
/// <https://schema.org/LikeAction>
pub const LIKE_ACTION_LABEL: &str = "LikeAction";
pub struct LikeActionIri;
impl PartialEq<&str> for LikeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIKE_ACTION_IRI_HTTP || *other == LIKE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<LikeActionIri> for &str {
	fn eq(&self, other: &LikeActionIri) -> bool {
		*self == LIKE_ACTION_IRI_HTTP || *self == LIKE_ACTION_IRI_HTTPS
	}
}
pub struct LikeActionIriOrLabel;
impl PartialEq<&str> for LikeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LikeActionIri || *other == LIKE_ACTION_LABEL
	}
}
impl PartialEq<LikeActionIriOrLabel> for &str {
	fn eq(&self, other: &LikeActionIriOrLabel) -> bool {
		*self == LikeActionIri || *self == LIKE_ACTION_LABEL
	}
}
