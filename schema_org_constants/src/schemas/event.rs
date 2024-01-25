/// <https://schema.org/Event>
pub const EVENT_IRI_HTTP: &str = "http://schema.org/Event";
/// <https://schema.org/Event>
pub const EVENT_IRI_HTTPS: &str = "https://schema.org/Event";
/// <https://schema.org/Event>
pub const EVENT_LABEL: &str = "Event";
pub struct EventIri;
impl PartialEq<&str> for EventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_IRI_HTTP || *other == EVENT_IRI_HTTPS
	}
}
impl PartialEq<EventIri> for &str {
	fn eq(&self, other: &EventIri) -> bool {
		*self == EVENT_IRI_HTTP || *self == EVENT_IRI_HTTPS
	}
}
pub struct EventIriOrLabel;
impl PartialEq<&str> for EventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventIri || *other == EVENT_LABEL
	}
}
impl PartialEq<EventIriOrLabel> for &str {
	fn eq(&self, other: &EventIriOrLabel) -> bool {
		*self == EventIri || *self == EVENT_LABEL
	}
}
