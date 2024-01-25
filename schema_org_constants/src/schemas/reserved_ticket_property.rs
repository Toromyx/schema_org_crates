/// <https://schema.org/reservedTicket>
pub const RESERVED_TICKET_PROPERTY_IRI_HTTP: &str = "http://schema.org/reservedTicket";
/// <https://schema.org/reservedTicket>
pub const RESERVED_TICKET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reservedTicket";
/// <https://schema.org/reservedTicket>
pub const RESERVED_TICKET_PROPERTY_LABEL: &str = "reservedTicket";
pub struct ReservedTicketPropertyIri;
impl PartialEq<&str> for ReservedTicketPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVED_TICKET_PROPERTY_IRI_HTTP || *other == RESERVED_TICKET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReservedTicketPropertyIri> for &str {
	fn eq(&self, other: &ReservedTicketPropertyIri) -> bool {
		*self == RESERVED_TICKET_PROPERTY_IRI_HTTP || *self == RESERVED_TICKET_PROPERTY_IRI_HTTPS
	}
}
pub struct ReservedTicketPropertyIriOrLabel;
impl PartialEq<&str> for ReservedTicketPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservedTicketPropertyIri || *other == RESERVED_TICKET_PROPERTY_LABEL
	}
}
impl PartialEq<ReservedTicketPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReservedTicketPropertyIriOrLabel) -> bool {
		*self == ReservedTicketPropertyIri || *self == RESERVED_TICKET_PROPERTY_LABEL
	}
}
