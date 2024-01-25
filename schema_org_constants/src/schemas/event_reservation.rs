/// <https://schema.org/EventReservation>
pub const EVENT_RESERVATION_IRI_HTTP: &str = "http://schema.org/EventReservation";
/// <https://schema.org/EventReservation>
pub const EVENT_RESERVATION_IRI_HTTPS: &str = "https://schema.org/EventReservation";
/// <https://schema.org/EventReservation>
pub const EVENT_RESERVATION_LABEL: &str = "EventReservation";
pub struct EventReservationIri;
impl PartialEq<&str> for EventReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_RESERVATION_IRI_HTTP || *other == EVENT_RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<EventReservationIri> for &str {
	fn eq(&self, other: &EventReservationIri) -> bool {
		*self == EVENT_RESERVATION_IRI_HTTP || *self == EVENT_RESERVATION_IRI_HTTPS
	}
}
pub struct EventReservationIriOrLabel;
impl PartialEq<&str> for EventReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventReservationIri || *other == EVENT_RESERVATION_LABEL
	}
}
impl PartialEq<EventReservationIriOrLabel> for &str {
	fn eq(&self, other: &EventReservationIriOrLabel) -> bool {
		*self == EventReservationIri || *self == EVENT_RESERVATION_LABEL
	}
}
