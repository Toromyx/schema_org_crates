/// <https://schema.org/replyToUrl>
pub const REPLY_TO_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/replyToUrl";
/// <https://schema.org/replyToUrl>
pub const REPLY_TO_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/replyToUrl";
/// <https://schema.org/replyToUrl>
pub const REPLY_TO_URL_PROPERTY_LABEL: &str = "replyToUrl";
pub struct ReplyToUrlPropertyIri;
impl PartialEq<&str> for ReplyToUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPLY_TO_URL_PROPERTY_IRI_HTTP || *other == REPLY_TO_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReplyToUrlPropertyIri> for &str {
	fn eq(&self, other: &ReplyToUrlPropertyIri) -> bool {
		*self == REPLY_TO_URL_PROPERTY_IRI_HTTP || *self == REPLY_TO_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct ReplyToUrlPropertyIriOrLabel;
impl PartialEq<&str> for ReplyToUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReplyToUrlPropertyIri || *other == REPLY_TO_URL_PROPERTY_LABEL
	}
}
impl PartialEq<ReplyToUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReplyToUrlPropertyIriOrLabel) -> bool {
		*self == ReplyToUrlPropertyIri || *self == REPLY_TO_URL_PROPERTY_LABEL
	}
}
