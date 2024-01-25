/// <https://schema.org/eventStatus>
pub const EVENT_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/eventStatus";
/// <https://schema.org/eventStatus>
pub const EVENT_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/eventStatus";
/// <https://schema.org/eventStatus>
pub const EVENT_STATUS_PROPERTY_LABEL: &str = "eventStatus";
pub struct EventStatusPropertyIri;
impl PartialEq<&str> for EventStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_STATUS_PROPERTY_IRI_HTTP || *other == EVENT_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EventStatusPropertyIri> for &str {
	fn eq(&self, other: &EventStatusPropertyIri) -> bool {
		*self == EVENT_STATUS_PROPERTY_IRI_HTTP || *self == EVENT_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct EventStatusPropertyIriOrLabel;
impl PartialEq<&str> for EventStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventStatusPropertyIri || *other == EVENT_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<EventStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EventStatusPropertyIriOrLabel) -> bool {
		*self == EventStatusPropertyIri || *self == EVENT_STATUS_PROPERTY_LABEL
	}
}
