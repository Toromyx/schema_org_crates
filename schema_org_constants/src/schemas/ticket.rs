/// <https://schema.org/Ticket>
pub const TICKET_IRI_HTTP: &str = "http://schema.org/Ticket";
/// <https://schema.org/Ticket>
pub const TICKET_IRI_HTTPS: &str = "https://schema.org/Ticket";
/// <https://schema.org/Ticket>
pub const TICKET_LABEL: &str = "Ticket";
pub struct TicketIri;
impl PartialEq<&str> for TicketIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TICKET_IRI_HTTP || *other == TICKET_IRI_HTTPS
	}
}
impl PartialEq<TicketIri> for &str {
	fn eq(&self, other: &TicketIri) -> bool {
		*self == TICKET_IRI_HTTP || *self == TICKET_IRI_HTTPS
	}
}
pub struct TicketIriOrLabel;
impl PartialEq<&str> for TicketIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TicketIri || *other == TICKET_LABEL
	}
}
impl PartialEq<TicketIriOrLabel> for &str {
	fn eq(&self, other: &TicketIriOrLabel) -> bool {
		*self == TicketIri || *self == TICKET_LABEL
	}
}
