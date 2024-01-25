/// <https://schema.org/EventScheduled>
pub const EVENT_SCHEDULED_IRI_HTTP: &str = "http://schema.org/EventScheduled";
/// <https://schema.org/EventScheduled>
pub const EVENT_SCHEDULED_IRI_HTTPS: &str = "https://schema.org/EventScheduled";
/// <https://schema.org/EventScheduled>
pub const EVENT_SCHEDULED_LABEL: &str = "EventScheduled";
pub struct EventScheduledIri;
impl PartialEq<&str> for EventScheduledIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_SCHEDULED_IRI_HTTP || *other == EVENT_SCHEDULED_IRI_HTTPS
	}
}
impl PartialEq<EventScheduledIri> for &str {
	fn eq(&self, other: &EventScheduledIri) -> bool {
		*self == EVENT_SCHEDULED_IRI_HTTP || *self == EVENT_SCHEDULED_IRI_HTTPS
	}
}
pub struct EventScheduledIriOrLabel;
impl PartialEq<&str> for EventScheduledIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventScheduledIri || *other == EVENT_SCHEDULED_LABEL
	}
}
impl PartialEq<EventScheduledIriOrLabel> for &str {
	fn eq(&self, other: &EventScheduledIriOrLabel) -> bool {
		*self == EventScheduledIri || *self == EVENT_SCHEDULED_LABEL
	}
}
