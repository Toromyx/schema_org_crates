/// <https://schema.org/MediaSubscription>
pub const MEDIA_SUBSCRIPTION_IRI_HTTP: &str = "http://schema.org/MediaSubscription";
/// <https://schema.org/MediaSubscription>
pub const MEDIA_SUBSCRIPTION_IRI_HTTPS: &str = "https://schema.org/MediaSubscription";
/// <https://schema.org/MediaSubscription>
pub const MEDIA_SUBSCRIPTION_LABEL: &str = "MediaSubscription";
pub struct MediaSubscriptionIri;
impl PartialEq<&str> for MediaSubscriptionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIA_SUBSCRIPTION_IRI_HTTP || *other == MEDIA_SUBSCRIPTION_IRI_HTTPS
	}
}
impl PartialEq<MediaSubscriptionIri> for &str {
	fn eq(&self, other: &MediaSubscriptionIri) -> bool {
		*self == MEDIA_SUBSCRIPTION_IRI_HTTP || *self == MEDIA_SUBSCRIPTION_IRI_HTTPS
	}
}
pub struct MediaSubscriptionIriOrLabel;
impl PartialEq<&str> for MediaSubscriptionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MediaSubscriptionIri || *other == MEDIA_SUBSCRIPTION_LABEL
	}
}
impl PartialEq<MediaSubscriptionIriOrLabel> for &str {
	fn eq(&self, other: &MediaSubscriptionIriOrLabel) -> bool {
		*self == MediaSubscriptionIri || *self == MEDIA_SUBSCRIPTION_LABEL
	}
}
