/// <https://schema.org/ticketNumber>
pub const TICKET_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/ticketNumber";
/// <https://schema.org/ticketNumber>
pub const TICKET_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ticketNumber";
/// <https://schema.org/ticketNumber>
pub const TICKET_NUMBER_PROPERTY_LABEL: &str = "ticketNumber";
pub struct TicketNumberPropertyIri;
impl PartialEq<&str> for TicketNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TICKET_NUMBER_PROPERTY_IRI_HTTP || *other == TICKET_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TicketNumberPropertyIri> for &str {
	fn eq(&self, other: &TicketNumberPropertyIri) -> bool {
		*self == TICKET_NUMBER_PROPERTY_IRI_HTTP || *self == TICKET_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct TicketNumberPropertyIriOrLabel;
impl PartialEq<&str> for TicketNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TicketNumberPropertyIri || *other == TICKET_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<TicketNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TicketNumberPropertyIriOrLabel) -> bool {
		*self == TicketNumberPropertyIri || *self == TICKET_NUMBER_PROPERTY_LABEL
	}
}
