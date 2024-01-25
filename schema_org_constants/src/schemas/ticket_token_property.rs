/// <https://schema.org/ticketToken>
pub const TICKET_TOKEN_PROPERTY_IRI_HTTP: &str = "http://schema.org/ticketToken";
/// <https://schema.org/ticketToken>
pub const TICKET_TOKEN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ticketToken";
/// <https://schema.org/ticketToken>
pub const TICKET_TOKEN_PROPERTY_LABEL: &str = "ticketToken";
pub struct TicketTokenPropertyIri;
impl PartialEq<&str> for TicketTokenPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TICKET_TOKEN_PROPERTY_IRI_HTTP || *other == TICKET_TOKEN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TicketTokenPropertyIri> for &str {
	fn eq(&self, other: &TicketTokenPropertyIri) -> bool {
		*self == TICKET_TOKEN_PROPERTY_IRI_HTTP || *self == TICKET_TOKEN_PROPERTY_IRI_HTTPS
	}
}
pub struct TicketTokenPropertyIriOrLabel;
impl PartialEq<&str> for TicketTokenPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TicketTokenPropertyIri || *other == TICKET_TOKEN_PROPERTY_LABEL
	}
}
impl PartialEq<TicketTokenPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TicketTokenPropertyIriOrLabel) -> bool {
		*self == TicketTokenPropertyIri || *self == TICKET_TOKEN_PROPERTY_LABEL
	}
}
