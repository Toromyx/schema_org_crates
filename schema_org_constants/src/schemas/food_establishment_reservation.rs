/// <https://schema.org/FoodEstablishmentReservation>
pub const FOOD_ESTABLISHMENT_RESERVATION_IRI_HTTP: &str =
	"http://schema.org/FoodEstablishmentReservation";
/// <https://schema.org/FoodEstablishmentReservation>
pub const FOOD_ESTABLISHMENT_RESERVATION_IRI_HTTPS: &str =
	"https://schema.org/FoodEstablishmentReservation";
/// <https://schema.org/FoodEstablishmentReservation>
pub const FOOD_ESTABLISHMENT_RESERVATION_LABEL: &str = "FoodEstablishmentReservation";
pub struct FoodEstablishmentReservationIri;
impl PartialEq<&str> for FoodEstablishmentReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOOD_ESTABLISHMENT_RESERVATION_IRI_HTTP
			|| *other == FOOD_ESTABLISHMENT_RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<FoodEstablishmentReservationIri> for &str {
	fn eq(&self, other: &FoodEstablishmentReservationIri) -> bool {
		*self == FOOD_ESTABLISHMENT_RESERVATION_IRI_HTTP
			|| *self == FOOD_ESTABLISHMENT_RESERVATION_IRI_HTTPS
	}
}
pub struct FoodEstablishmentReservationIriOrLabel;
impl PartialEq<&str> for FoodEstablishmentReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoodEstablishmentReservationIri || *other == FOOD_ESTABLISHMENT_RESERVATION_LABEL
	}
}
impl PartialEq<FoodEstablishmentReservationIriOrLabel> for &str {
	fn eq(&self, other: &FoodEstablishmentReservationIriOrLabel) -> bool {
		*self == FoodEstablishmentReservationIri || *self == FOOD_ESTABLISHMENT_RESERVATION_LABEL
	}
}
