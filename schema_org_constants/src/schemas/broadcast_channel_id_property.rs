/// <https://schema.org/broadcastChannelId>
pub const BROADCAST_CHANNEL_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/broadcastChannelId";
/// <https://schema.org/broadcastChannelId>
pub const BROADCAST_CHANNEL_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/broadcastChannelId";
/// <https://schema.org/broadcastChannelId>
pub const BROADCAST_CHANNEL_ID_PROPERTY_LABEL: &str = "broadcastChannelId";
pub struct BroadcastChannelIdPropertyIri;
impl PartialEq<&str> for BroadcastChannelIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_CHANNEL_ID_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_CHANNEL_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastChannelIdPropertyIri> for &str {
	fn eq(&self, other: &BroadcastChannelIdPropertyIri) -> bool {
		*self == BROADCAST_CHANNEL_ID_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_CHANNEL_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastChannelIdPropertyIriOrLabel;
impl PartialEq<&str> for BroadcastChannelIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastChannelIdPropertyIri || *other == BROADCAST_CHANNEL_ID_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastChannelIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastChannelIdPropertyIriOrLabel) -> bool {
		*self == BroadcastChannelIdPropertyIri || *self == BROADCAST_CHANNEL_ID_PROPERTY_LABEL
	}
}
