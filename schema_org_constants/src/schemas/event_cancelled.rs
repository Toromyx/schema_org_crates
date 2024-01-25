/// <https://schema.org/EventCancelled>
pub const EVENT_CANCELLED_IRI_HTTP: &str = "http://schema.org/EventCancelled";
/// <https://schema.org/EventCancelled>
pub const EVENT_CANCELLED_IRI_HTTPS: &str = "https://schema.org/EventCancelled";
/// <https://schema.org/EventCancelled>
pub const EVENT_CANCELLED_LABEL: &str = "EventCancelled";
pub struct EventCancelledIri;
impl PartialEq<&str> for EventCancelledIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_CANCELLED_IRI_HTTP || *other == EVENT_CANCELLED_IRI_HTTPS
	}
}
impl PartialEq<EventCancelledIri> for &str {
	fn eq(&self, other: &EventCancelledIri) -> bool {
		*self == EVENT_CANCELLED_IRI_HTTP || *self == EVENT_CANCELLED_IRI_HTTPS
	}
}
pub struct EventCancelledIriOrLabel;
impl PartialEq<&str> for EventCancelledIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventCancelledIri || *other == EVENT_CANCELLED_LABEL
	}
}
impl PartialEq<EventCancelledIriOrLabel> for &str {
	fn eq(&self, other: &EventCancelledIriOrLabel) -> bool {
		*self == EventCancelledIri || *self == EVENT_CANCELLED_LABEL
	}
}
