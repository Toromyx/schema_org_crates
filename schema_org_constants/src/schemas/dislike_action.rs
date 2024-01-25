/// <https://schema.org/DislikeAction>
pub const DISLIKE_ACTION_IRI_HTTP: &str = "http://schema.org/DislikeAction";
/// <https://schema.org/DislikeAction>
pub const DISLIKE_ACTION_IRI_HTTPS: &str = "https://schema.org/DislikeAction";
/// <https://schema.org/DislikeAction>
pub const DISLIKE_ACTION_LABEL: &str = "DislikeAction";
pub struct DislikeActionIri;
impl PartialEq<&str> for DislikeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISLIKE_ACTION_IRI_HTTP || *other == DISLIKE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DislikeActionIri> for &str {
	fn eq(&self, other: &DislikeActionIri) -> bool {
		*self == DISLIKE_ACTION_IRI_HTTP || *self == DISLIKE_ACTION_IRI_HTTPS
	}
}
pub struct DislikeActionIriOrLabel;
impl PartialEq<&str> for DislikeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DislikeActionIri || *other == DISLIKE_ACTION_LABEL
	}
}
impl PartialEq<DislikeActionIriOrLabel> for &str {
	fn eq(&self, other: &DislikeActionIriOrLabel) -> bool {
		*self == DislikeActionIri || *self == DISLIKE_ACTION_LABEL
	}
}
