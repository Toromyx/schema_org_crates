/// <https://schema.org/tourBookingPage>
pub const TOUR_BOOKING_PAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/tourBookingPage";
/// <https://schema.org/tourBookingPage>
pub const TOUR_BOOKING_PAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/tourBookingPage";
/// <https://schema.org/tourBookingPage>
pub const TOUR_BOOKING_PAGE_PROPERTY_LABEL: &str = "tourBookingPage";
pub struct TourBookingPagePropertyIri;
impl PartialEq<&str> for TourBookingPagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOUR_BOOKING_PAGE_PROPERTY_IRI_HTTP
			|| *other == TOUR_BOOKING_PAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TourBookingPagePropertyIri> for &str {
	fn eq(&self, other: &TourBookingPagePropertyIri) -> bool {
		*self == TOUR_BOOKING_PAGE_PROPERTY_IRI_HTTP
			|| *self == TOUR_BOOKING_PAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct TourBookingPagePropertyIriOrLabel;
impl PartialEq<&str> for TourBookingPagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TourBookingPagePropertyIri || *other == TOUR_BOOKING_PAGE_PROPERTY_LABEL
	}
}
impl PartialEq<TourBookingPagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TourBookingPagePropertyIriOrLabel) -> bool {
		*self == TourBookingPagePropertyIri || *self == TOUR_BOOKING_PAGE_PROPERTY_LABEL
	}
}
