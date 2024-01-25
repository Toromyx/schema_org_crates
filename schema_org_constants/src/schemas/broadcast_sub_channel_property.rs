/// <https://schema.org/broadcastSubChannel>
pub const BROADCAST_SUB_CHANNEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/broadcastSubChannel";
/// <https://schema.org/broadcastSubChannel>
pub const BROADCAST_SUB_CHANNEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/broadcastSubChannel";
/// <https://schema.org/broadcastSubChannel>
pub const BROADCAST_SUB_CHANNEL_PROPERTY_LABEL: &str = "broadcastSubChannel";
pub struct BroadcastSubChannelPropertyIri;
impl PartialEq<&str> for BroadcastSubChannelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_SUB_CHANNEL_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_SUB_CHANNEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastSubChannelPropertyIri> for &str {
	fn eq(&self, other: &BroadcastSubChannelPropertyIri) -> bool {
		*self == BROADCAST_SUB_CHANNEL_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_SUB_CHANNEL_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastSubChannelPropertyIriOrLabel;
impl PartialEq<&str> for BroadcastSubChannelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastSubChannelPropertyIri || *other == BROADCAST_SUB_CHANNEL_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastSubChannelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastSubChannelPropertyIriOrLabel) -> bool {
		*self == BroadcastSubChannelPropertyIri || *self == BROADCAST_SUB_CHANNEL_PROPERTY_LABEL
	}
}
