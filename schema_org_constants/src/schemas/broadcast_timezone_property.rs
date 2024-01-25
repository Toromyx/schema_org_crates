/// <https://schema.org/broadcastTimezone>
pub const BROADCAST_TIMEZONE_PROPERTY_IRI_HTTP: &str = "http://schema.org/broadcastTimezone";
/// <https://schema.org/broadcastTimezone>
pub const BROADCAST_TIMEZONE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/broadcastTimezone";
/// <https://schema.org/broadcastTimezone>
pub const BROADCAST_TIMEZONE_PROPERTY_LABEL: &str = "broadcastTimezone";
pub struct BroadcastTimezonePropertyIri;
impl PartialEq<&str> for BroadcastTimezonePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_TIMEZONE_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_TIMEZONE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastTimezonePropertyIri> for &str {
	fn eq(&self, other: &BroadcastTimezonePropertyIri) -> bool {
		*self == BROADCAST_TIMEZONE_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_TIMEZONE_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastTimezonePropertyIriOrLabel;
impl PartialEq<&str> for BroadcastTimezonePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastTimezonePropertyIri || *other == BROADCAST_TIMEZONE_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastTimezonePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastTimezonePropertyIriOrLabel) -> bool {
		*self == BroadcastTimezonePropertyIri || *self == BROADCAST_TIMEZONE_PROPERTY_LABEL
	}
}
