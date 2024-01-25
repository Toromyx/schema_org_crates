/// <https://schema.org/events>
#[deprecated = "This schema is superseded by <https://schema.org/event>."]
pub const EVENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/events";
/// <https://schema.org/events>
#[deprecated = "This schema is superseded by <https://schema.org/event>."]
pub const EVENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/events";
/// <https://schema.org/events>
#[deprecated = "This schema is superseded by <https://schema.org/event>."]
pub const EVENTS_PROPERTY_LABEL: &str = "events";
pub struct EventsPropertyIri;
impl PartialEq<&str> for EventsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENTS_PROPERTY_IRI_HTTP || *other == EVENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EventsPropertyIri> for &str {
	fn eq(&self, other: &EventsPropertyIri) -> bool {
		*self == EVENTS_PROPERTY_IRI_HTTP || *self == EVENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct EventsPropertyIriOrLabel;
impl PartialEq<&str> for EventsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventsPropertyIri || *other == EVENTS_PROPERTY_LABEL
	}
}
impl PartialEq<EventsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EventsPropertyIriOrLabel) -> bool {
		*self == EventsPropertyIri || *self == EVENTS_PROPERTY_LABEL
	}
}
