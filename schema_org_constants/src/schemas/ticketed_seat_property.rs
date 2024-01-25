/// <https://schema.org/ticketedSeat>
pub const TICKETED_SEAT_PROPERTY_IRI_HTTP: &str = "http://schema.org/ticketedSeat";
/// <https://schema.org/ticketedSeat>
pub const TICKETED_SEAT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ticketedSeat";
/// <https://schema.org/ticketedSeat>
pub const TICKETED_SEAT_PROPERTY_LABEL: &str = "ticketedSeat";
pub struct TicketedSeatPropertyIri;
impl PartialEq<&str> for TicketedSeatPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TICKETED_SEAT_PROPERTY_IRI_HTTP || *other == TICKETED_SEAT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TicketedSeatPropertyIri> for &str {
	fn eq(&self, other: &TicketedSeatPropertyIri) -> bool {
		*self == TICKETED_SEAT_PROPERTY_IRI_HTTP || *self == TICKETED_SEAT_PROPERTY_IRI_HTTPS
	}
}
pub struct TicketedSeatPropertyIriOrLabel;
impl PartialEq<&str> for TicketedSeatPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TicketedSeatPropertyIri || *other == TICKETED_SEAT_PROPERTY_LABEL
	}
}
impl PartialEq<TicketedSeatPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TicketedSeatPropertyIriOrLabel) -> bool {
		*self == TicketedSeatPropertyIri || *self == TICKETED_SEAT_PROPERTY_LABEL
	}
}
