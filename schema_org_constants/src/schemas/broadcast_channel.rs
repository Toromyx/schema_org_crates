/// <https://schema.org/BroadcastChannel>
pub const BROADCAST_CHANNEL_IRI_HTTP: &str = "http://schema.org/BroadcastChannel";
/// <https://schema.org/BroadcastChannel>
pub const BROADCAST_CHANNEL_IRI_HTTPS: &str = "https://schema.org/BroadcastChannel";
/// <https://schema.org/BroadcastChannel>
pub const BROADCAST_CHANNEL_LABEL: &str = "BroadcastChannel";
pub struct BroadcastChannelIri;
impl PartialEq<&str> for BroadcastChannelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_CHANNEL_IRI_HTTP || *other == BROADCAST_CHANNEL_IRI_HTTPS
	}
}
impl PartialEq<BroadcastChannelIri> for &str {
	fn eq(&self, other: &BroadcastChannelIri) -> bool {
		*self == BROADCAST_CHANNEL_IRI_HTTP || *self == BROADCAST_CHANNEL_IRI_HTTPS
	}
}
pub struct BroadcastChannelIriOrLabel;
impl PartialEq<&str> for BroadcastChannelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastChannelIri || *other == BROADCAST_CHANNEL_LABEL
	}
}
impl PartialEq<BroadcastChannelIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastChannelIriOrLabel) -> bool {
		*self == BroadcastChannelIri || *self == BROADCAST_CHANNEL_LABEL
	}
}
