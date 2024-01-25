/// <https://schema.org/hasBroadcastChannel>
pub const HAS_BROADCAST_CHANNEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasBroadcastChannel";
/// <https://schema.org/hasBroadcastChannel>
pub const HAS_BROADCAST_CHANNEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasBroadcastChannel";
/// <https://schema.org/hasBroadcastChannel>
pub const HAS_BROADCAST_CHANNEL_PROPERTY_LABEL: &str = "hasBroadcastChannel";
pub struct HasBroadcastChannelPropertyIri;
impl PartialEq<&str> for HasBroadcastChannelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_BROADCAST_CHANNEL_PROPERTY_IRI_HTTP
			|| *other == HAS_BROADCAST_CHANNEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasBroadcastChannelPropertyIri> for &str {
	fn eq(&self, other: &HasBroadcastChannelPropertyIri) -> bool {
		*self == HAS_BROADCAST_CHANNEL_PROPERTY_IRI_HTTP
			|| *self == HAS_BROADCAST_CHANNEL_PROPERTY_IRI_HTTPS
	}
}
pub struct HasBroadcastChannelPropertyIriOrLabel;
impl PartialEq<&str> for HasBroadcastChannelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasBroadcastChannelPropertyIri || *other == HAS_BROADCAST_CHANNEL_PROPERTY_LABEL
	}
}
impl PartialEq<HasBroadcastChannelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasBroadcastChannelPropertyIriOrLabel) -> bool {
		*self == HasBroadcastChannelPropertyIri || *self == HAS_BROADCAST_CHANNEL_PROPERTY_LABEL
	}
}
