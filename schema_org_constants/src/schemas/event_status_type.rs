/// <https://schema.org/EventStatusType>
pub const EVENT_STATUS_TYPE_IRI_HTTP: &str = "http://schema.org/EventStatusType";
/// <https://schema.org/EventStatusType>
pub const EVENT_STATUS_TYPE_IRI_HTTPS: &str = "https://schema.org/EventStatusType";
/// <https://schema.org/EventStatusType>
pub const EVENT_STATUS_TYPE_LABEL: &str = "EventStatusType";
pub struct EventStatusTypeIri;
impl PartialEq<&str> for EventStatusTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_STATUS_TYPE_IRI_HTTP || *other == EVENT_STATUS_TYPE_IRI_HTTPS
	}
}
impl PartialEq<EventStatusTypeIri> for &str {
	fn eq(&self, other: &EventStatusTypeIri) -> bool {
		*self == EVENT_STATUS_TYPE_IRI_HTTP || *self == EVENT_STATUS_TYPE_IRI_HTTPS
	}
}
pub struct EventStatusTypeIriOrLabel;
impl PartialEq<&str> for EventStatusTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventStatusTypeIri || *other == EVENT_STATUS_TYPE_LABEL
	}
}
impl PartialEq<EventStatusTypeIriOrLabel> for &str {
	fn eq(&self, other: &EventStatusTypeIriOrLabel) -> bool {
		*self == EventStatusTypeIri || *self == EVENT_STATUS_TYPE_LABEL
	}
}
