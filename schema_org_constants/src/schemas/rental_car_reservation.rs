/// <https://schema.org/RentalCarReservation>
pub const RENTAL_CAR_RESERVATION_IRI_HTTP: &str = "http://schema.org/RentalCarReservation";
/// <https://schema.org/RentalCarReservation>
pub const RENTAL_CAR_RESERVATION_IRI_HTTPS: &str = "https://schema.org/RentalCarReservation";
/// <https://schema.org/RentalCarReservation>
pub const RENTAL_CAR_RESERVATION_LABEL: &str = "RentalCarReservation";
pub struct RentalCarReservationIri;
impl PartialEq<&str> for RentalCarReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RENTAL_CAR_RESERVATION_IRI_HTTP || *other == RENTAL_CAR_RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<RentalCarReservationIri> for &str {
	fn eq(&self, other: &RentalCarReservationIri) -> bool {
		*self == RENTAL_CAR_RESERVATION_IRI_HTTP || *self == RENTAL_CAR_RESERVATION_IRI_HTTPS
	}
}
pub struct RentalCarReservationIriOrLabel;
impl PartialEq<&str> for RentalCarReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RentalCarReservationIri || *other == RENTAL_CAR_RESERVATION_LABEL
	}
}
impl PartialEq<RentalCarReservationIriOrLabel> for &str {
	fn eq(&self, other: &RentalCarReservationIriOrLabel) -> bool {
		*self == RentalCarReservationIri || *self == RENTAL_CAR_RESERVATION_LABEL
	}
}
