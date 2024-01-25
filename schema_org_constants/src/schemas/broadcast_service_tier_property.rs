/// <https://schema.org/broadcastServiceTier>
pub const BROADCAST_SERVICE_TIER_PROPERTY_IRI_HTTP: &str = "http://schema.org/broadcastServiceTier";
/// <https://schema.org/broadcastServiceTier>
pub const BROADCAST_SERVICE_TIER_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/broadcastServiceTier";
/// <https://schema.org/broadcastServiceTier>
pub const BROADCAST_SERVICE_TIER_PROPERTY_LABEL: &str = "broadcastServiceTier";
pub struct BroadcastServiceTierPropertyIri;
impl PartialEq<&str> for BroadcastServiceTierPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_SERVICE_TIER_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_SERVICE_TIER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastServiceTierPropertyIri> for &str {
	fn eq(&self, other: &BroadcastServiceTierPropertyIri) -> bool {
		*self == BROADCAST_SERVICE_TIER_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_SERVICE_TIER_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastServiceTierPropertyIriOrLabel;
impl PartialEq<&str> for BroadcastServiceTierPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastServiceTierPropertyIri || *other == BROADCAST_SERVICE_TIER_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastServiceTierPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastServiceTierPropertyIriOrLabel) -> bool {
		*self == BroadcastServiceTierPropertyIri || *self == BROADCAST_SERVICE_TIER_PROPERTY_LABEL
	}
}
