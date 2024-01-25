/// <https://schema.org/EventPostponed>
pub const EVENT_POSTPONED_IRI_HTTP: &str = "http://schema.org/EventPostponed";
/// <https://schema.org/EventPostponed>
pub const EVENT_POSTPONED_IRI_HTTPS: &str = "https://schema.org/EventPostponed";
/// <https://schema.org/EventPostponed>
pub const EVENT_POSTPONED_LABEL: &str = "EventPostponed";
pub struct EventPostponedIri;
impl PartialEq<&str> for EventPostponedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_POSTPONED_IRI_HTTP || *other == EVENT_POSTPONED_IRI_HTTPS
	}
}
impl PartialEq<EventPostponedIri> for &str {
	fn eq(&self, other: &EventPostponedIri) -> bool {
		*self == EVENT_POSTPONED_IRI_HTTP || *self == EVENT_POSTPONED_IRI_HTTPS
	}
}
pub struct EventPostponedIriOrLabel;
impl PartialEq<&str> for EventPostponedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventPostponedIri || *other == EVENT_POSTPONED_LABEL
	}
}
impl PartialEq<EventPostponedIriOrLabel> for &str {
	fn eq(&self, other: &EventPostponedIriOrLabel) -> bool {
		*self == EventPostponedIri || *self == EVENT_POSTPONED_LABEL
	}
}
