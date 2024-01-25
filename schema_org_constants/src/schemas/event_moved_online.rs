/// <https://schema.org/EventMovedOnline>
pub const EVENT_MOVED_ONLINE_IRI_HTTP: &str = "http://schema.org/EventMovedOnline";
/// <https://schema.org/EventMovedOnline>
pub const EVENT_MOVED_ONLINE_IRI_HTTPS: &str = "https://schema.org/EventMovedOnline";
/// <https://schema.org/EventMovedOnline>
pub const EVENT_MOVED_ONLINE_LABEL: &str = "EventMovedOnline";
pub struct EventMovedOnlineIri;
impl PartialEq<&str> for EventMovedOnlineIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_MOVED_ONLINE_IRI_HTTP || *other == EVENT_MOVED_ONLINE_IRI_HTTPS
	}
}
impl PartialEq<EventMovedOnlineIri> for &str {
	fn eq(&self, other: &EventMovedOnlineIri) -> bool {
		*self == EVENT_MOVED_ONLINE_IRI_HTTP || *self == EVENT_MOVED_ONLINE_IRI_HTTPS
	}
}
pub struct EventMovedOnlineIriOrLabel;
impl PartialEq<&str> for EventMovedOnlineIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventMovedOnlineIri || *other == EVENT_MOVED_ONLINE_LABEL
	}
}
impl PartialEq<EventMovedOnlineIriOrLabel> for &str {
	fn eq(&self, other: &EventMovedOnlineIriOrLabel) -> bool {
		*self == EventMovedOnlineIri || *self == EVENT_MOVED_ONLINE_LABEL
	}
}
