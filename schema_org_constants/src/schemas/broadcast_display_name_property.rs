/// <https://schema.org/broadcastDisplayName>
pub const BROADCAST_DISPLAY_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/broadcastDisplayName";
/// <https://schema.org/broadcastDisplayName>
pub const BROADCAST_DISPLAY_NAME_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/broadcastDisplayName";
/// <https://schema.org/broadcastDisplayName>
pub const BROADCAST_DISPLAY_NAME_PROPERTY_LABEL: &str = "broadcastDisplayName";
pub struct BroadcastDisplayNamePropertyIri;
impl PartialEq<&str> for BroadcastDisplayNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_DISPLAY_NAME_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_DISPLAY_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastDisplayNamePropertyIri> for &str {
	fn eq(&self, other: &BroadcastDisplayNamePropertyIri) -> bool {
		*self == BROADCAST_DISPLAY_NAME_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_DISPLAY_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastDisplayNamePropertyIriOrLabel;
impl PartialEq<&str> for BroadcastDisplayNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastDisplayNamePropertyIri || *other == BROADCAST_DISPLAY_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastDisplayNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastDisplayNamePropertyIriOrLabel) -> bool {
		*self == BroadcastDisplayNamePropertyIri || *self == BROADCAST_DISPLAY_NAME_PROPERTY_LABEL
	}
}
