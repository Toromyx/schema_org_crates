/// <https://schema.org/bookingTime>
pub const BOOKING_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/bookingTime";
/// <https://schema.org/bookingTime>
pub const BOOKING_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bookingTime";
/// <https://schema.org/bookingTime>
pub const BOOKING_TIME_PROPERTY_LABEL: &str = "bookingTime";
pub struct BookingTimePropertyIri;
impl PartialEq<&str> for BookingTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOKING_TIME_PROPERTY_IRI_HTTP || *other == BOOKING_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BookingTimePropertyIri> for &str {
	fn eq(&self, other: &BookingTimePropertyIri) -> bool {
		*self == BOOKING_TIME_PROPERTY_IRI_HTTP || *self == BOOKING_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct BookingTimePropertyIriOrLabel;
impl PartialEq<&str> for BookingTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BookingTimePropertyIri || *other == BOOKING_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<BookingTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BookingTimePropertyIriOrLabel) -> bool {
		*self == BookingTimePropertyIri || *self == BOOKING_TIME_PROPERTY_LABEL
	}
}
