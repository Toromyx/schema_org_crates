/// <https://schema.org/bookingAgent>
#[deprecated = "This schema is superseded by <https://schema.org/broker>."]
pub const BOOKING_AGENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/bookingAgent";
/// <https://schema.org/bookingAgent>
#[deprecated = "This schema is superseded by <https://schema.org/broker>."]
pub const BOOKING_AGENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bookingAgent";
/// <https://schema.org/bookingAgent>
#[deprecated = "This schema is superseded by <https://schema.org/broker>."]
pub const BOOKING_AGENT_PROPERTY_LABEL: &str = "bookingAgent";
pub struct BookingAgentPropertyIri;
impl PartialEq<&str> for BookingAgentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOKING_AGENT_PROPERTY_IRI_HTTP || *other == BOOKING_AGENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BookingAgentPropertyIri> for &str {
	fn eq(&self, other: &BookingAgentPropertyIri) -> bool {
		*self == BOOKING_AGENT_PROPERTY_IRI_HTTP || *self == BOOKING_AGENT_PROPERTY_IRI_HTTPS
	}
}
pub struct BookingAgentPropertyIriOrLabel;
impl PartialEq<&str> for BookingAgentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BookingAgentPropertyIri || *other == BOOKING_AGENT_PROPERTY_LABEL
	}
}
impl PartialEq<BookingAgentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BookingAgentPropertyIriOrLabel) -> bool {
		*self == BookingAgentPropertyIri || *self == BOOKING_AGENT_PROPERTY_LABEL
	}
}
