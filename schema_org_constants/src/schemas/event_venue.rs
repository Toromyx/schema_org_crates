/// <https://schema.org/EventVenue>
pub const EVENT_VENUE_IRI_HTTP: &str = "http://schema.org/EventVenue";
/// <https://schema.org/EventVenue>
pub const EVENT_VENUE_IRI_HTTPS: &str = "https://schema.org/EventVenue";
/// <https://schema.org/EventVenue>
pub const EVENT_VENUE_LABEL: &str = "EventVenue";
pub struct EventVenueIri;
impl PartialEq<&str> for EventVenueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_VENUE_IRI_HTTP || *other == EVENT_VENUE_IRI_HTTPS
	}
}
impl PartialEq<EventVenueIri> for &str {
	fn eq(&self, other: &EventVenueIri) -> bool {
		*self == EVENT_VENUE_IRI_HTTP || *self == EVENT_VENUE_IRI_HTTPS
	}
}
pub struct EventVenueIriOrLabel;
impl PartialEq<&str> for EventVenueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventVenueIri || *other == EVENT_VENUE_LABEL
	}
}
impl PartialEq<EventVenueIriOrLabel> for &str {
	fn eq(&self, other: &EventVenueIriOrLabel) -> bool {
		*self == EventVenueIri || *self == EVENT_VENUE_LABEL
	}
}
