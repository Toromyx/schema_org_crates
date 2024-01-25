/// <https://schema.org/EventRescheduled>
pub const EVENT_RESCHEDULED_IRI_HTTP: &str = "http://schema.org/EventRescheduled";
/// <https://schema.org/EventRescheduled>
pub const EVENT_RESCHEDULED_IRI_HTTPS: &str = "https://schema.org/EventRescheduled";
/// <https://schema.org/EventRescheduled>
pub const EVENT_RESCHEDULED_LABEL: &str = "EventRescheduled";
pub struct EventRescheduledIri;
impl PartialEq<&str> for EventRescheduledIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_RESCHEDULED_IRI_HTTP || *other == EVENT_RESCHEDULED_IRI_HTTPS
	}
}
impl PartialEq<EventRescheduledIri> for &str {
	fn eq(&self, other: &EventRescheduledIri) -> bool {
		*self == EVENT_RESCHEDULED_IRI_HTTP || *self == EVENT_RESCHEDULED_IRI_HTTPS
	}
}
pub struct EventRescheduledIriOrLabel;
impl PartialEq<&str> for EventRescheduledIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventRescheduledIri || *other == EVENT_RESCHEDULED_LABEL
	}
}
impl PartialEq<EventRescheduledIriOrLabel> for &str {
	fn eq(&self, other: &EventRescheduledIriOrLabel) -> bool {
		*self == EventRescheduledIri || *self == EVENT_RESCHEDULED_LABEL
	}
}
