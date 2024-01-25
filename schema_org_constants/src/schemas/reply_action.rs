/// <https://schema.org/ReplyAction>
pub const REPLY_ACTION_IRI_HTTP: &str = "http://schema.org/ReplyAction";
/// <https://schema.org/ReplyAction>
pub const REPLY_ACTION_IRI_HTTPS: &str = "https://schema.org/ReplyAction";
/// <https://schema.org/ReplyAction>
pub const REPLY_ACTION_LABEL: &str = "ReplyAction";
pub struct ReplyActionIri;
impl PartialEq<&str> for ReplyActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPLY_ACTION_IRI_HTTP || *other == REPLY_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ReplyActionIri> for &str {
	fn eq(&self, other: &ReplyActionIri) -> bool {
		*self == REPLY_ACTION_IRI_HTTP || *self == REPLY_ACTION_IRI_HTTPS
	}
}
pub struct ReplyActionIriOrLabel;
impl PartialEq<&str> for ReplyActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReplyActionIri || *other == REPLY_ACTION_LABEL
	}
}
impl PartialEq<ReplyActionIriOrLabel> for &str {
	fn eq(&self, other: &ReplyActionIriOrLabel) -> bool {
		*self == ReplyActionIri || *self == REPLY_ACTION_LABEL
	}
}
