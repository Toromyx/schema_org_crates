/// <https://schema.org/event>
pub const EVENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/event";
/// <https://schema.org/event>
pub const EVENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/event";
/// <https://schema.org/event>
pub const EVENT_PROPERTY_LABEL: &str = "event";
pub struct EventPropertyIri;
impl PartialEq<&str> for EventPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_PROPERTY_IRI_HTTP || *other == EVENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EventPropertyIri> for &str {
	fn eq(&self, other: &EventPropertyIri) -> bool {
		*self == EVENT_PROPERTY_IRI_HTTP || *self == EVENT_PROPERTY_IRI_HTTPS
	}
}
pub struct EventPropertyIriOrLabel;
impl PartialEq<&str> for EventPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventPropertyIri || *other == EVENT_PROPERTY_LABEL
	}
}
impl PartialEq<EventPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EventPropertyIriOrLabel) -> bool {
		*self == EventPropertyIri || *self == EVENT_PROPERTY_LABEL
	}
}
