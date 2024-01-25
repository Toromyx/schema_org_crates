/// <https://schema.org/BroadcastEvent>
pub const BROADCAST_EVENT_IRI_HTTP: &str = "http://schema.org/BroadcastEvent";
/// <https://schema.org/BroadcastEvent>
pub const BROADCAST_EVENT_IRI_HTTPS: &str = "https://schema.org/BroadcastEvent";
/// <https://schema.org/BroadcastEvent>
pub const BROADCAST_EVENT_LABEL: &str = "BroadcastEvent";
pub struct BroadcastEventIri;
impl PartialEq<&str> for BroadcastEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROADCAST_EVENT_IRI_HTTP || *other == BROADCAST_EVENT_IRI_HTTPS
	}
}
impl PartialEq<BroadcastEventIri> for &str {
	fn eq(&self, other: &BroadcastEventIri) -> bool {
		*self == BROADCAST_EVENT_IRI_HTTP || *self == BROADCAST_EVENT_IRI_HTTPS
	}
}
pub struct BroadcastEventIriOrLabel;
impl PartialEq<&str> for BroadcastEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BroadcastEventIri || *other == BROADCAST_EVENT_LABEL
	}
}
impl PartialEq<BroadcastEventIriOrLabel> for &str {
	fn eq(&self, other: &BroadcastEventIriOrLabel) -> bool {
		*self == BroadcastEventIri || *self == BROADCAST_EVENT_LABEL
	}
}
