/// <https://schema.org/broadcastAffiliateOf>
pub const BROADCAST_AFFILIATE_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/broadcastAffiliateOf";
/// <https://schema.org/broadcastAffiliateOf>
pub const BROADCAST_AFFILIATE_OF_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/broadcastAffiliateOf";
/// <https://schema.org/broadcastAffiliateOf>
pub const BROADCAST_AFFILIATE_OF_PROPERTY_LABEL: &str = "broadcastAffiliateOf";
pub struct BroadcastAffiliateOfPropertyIri;
impl PartialEq<&str> for BroadcastAffiliateOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_AFFILIATE_OF_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_AFFILIATE_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastAffiliateOfPropertyIri> for &str {
	fn eq(&self, other: &BroadcastAffiliateOfPropertyIri) -> bool {
		*self == BROADCAST_AFFILIATE_OF_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_AFFILIATE_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastAffiliateOfPropertyIriOrLabel;
impl PartialEq<&str> for BroadcastAffiliateOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastAffiliateOfPropertyIri || *other == BROADCAST_AFFILIATE_OF_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastAffiliateOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastAffiliateOfPropertyIriOrLabel) -> bool {
		*self == BroadcastAffiliateOfPropertyIri || *self == BROADCAST_AFFILIATE_OF_PROPERTY_LABEL
	}
}
