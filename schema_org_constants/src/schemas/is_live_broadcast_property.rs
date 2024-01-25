/// <https://schema.org/isLiveBroadcast>
pub const IS_LIVE_BROADCAST_PROPERTY_IRI_HTTP: &str = "http://schema.org/isLiveBroadcast";
/// <https://schema.org/isLiveBroadcast>
pub const IS_LIVE_BROADCAST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isLiveBroadcast";
/// <https://schema.org/isLiveBroadcast>
pub const IS_LIVE_BROADCAST_PROPERTY_LABEL: &str = "isLiveBroadcast";
pub struct IsLiveBroadcastPropertyIri;
impl PartialEq<&str> for IsLiveBroadcastPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_LIVE_BROADCAST_PROPERTY_IRI_HTTP
			|| *other == IS_LIVE_BROADCAST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsLiveBroadcastPropertyIri> for &str {
	fn eq(&self, other: &IsLiveBroadcastPropertyIri) -> bool {
		*self == IS_LIVE_BROADCAST_PROPERTY_IRI_HTTP
			|| *self == IS_LIVE_BROADCAST_PROPERTY_IRI_HTTPS
	}
}
pub struct IsLiveBroadcastPropertyIriOrLabel;
impl PartialEq<&str> for IsLiveBroadcastPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsLiveBroadcastPropertyIri || *other == IS_LIVE_BROADCAST_PROPERTY_LABEL
	}
}
impl PartialEq<IsLiveBroadcastPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsLiveBroadcastPropertyIriOrLabel) -> bool {
		*self == IsLiveBroadcastPropertyIri || *self == IS_LIVE_BROADCAST_PROPERTY_LABEL
	}
}
