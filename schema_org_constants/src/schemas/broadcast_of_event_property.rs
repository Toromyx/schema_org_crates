/// <https://schema.org/broadcastOfEvent>
pub const BROADCAST_OF_EVENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/broadcastOfEvent";
/// <https://schema.org/broadcastOfEvent>
pub const BROADCAST_OF_EVENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/broadcastOfEvent";
/// <https://schema.org/broadcastOfEvent>
pub const BROADCAST_OF_EVENT_PROPERTY_LABEL: &str = "broadcastOfEvent";
pub struct BroadcastOfEventPropertyIri;
impl PartialEq<&str> for BroadcastOfEventPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_OF_EVENT_PROPERTY_IRI_HTTP
			|| *other == BROADCAST_OF_EVENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BroadcastOfEventPropertyIri> for &str {
	fn eq(&self, other: &BroadcastOfEventPropertyIri) -> bool {
		*self == BROADCAST_OF_EVENT_PROPERTY_IRI_HTTP
			|| *self == BROADCAST_OF_EVENT_PROPERTY_IRI_HTTPS
	}
}
pub struct BroadcastOfEventPropertyIriOrLabel;
impl PartialEq<&str> for BroadcastOfEventPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastOfEventPropertyIri || *other == BROADCAST_OF_EVENT_PROPERTY_LABEL
	}
}
impl PartialEq<BroadcastOfEventPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastOfEventPropertyIriOrLabel) -> bool {
		*self == BroadcastOfEventPropertyIri || *self == BROADCAST_OF_EVENT_PROPERTY_LABEL
	}
}
