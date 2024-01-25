/// <https://schema.org/VacationRental>
pub const VACATION_RENTAL_IRI_HTTP: &str = "http://schema.org/VacationRental";
/// <https://schema.org/VacationRental>
pub const VACATION_RENTAL_IRI_HTTPS: &str = "https://schema.org/VacationRental";
/// <https://schema.org/VacationRental>
pub const VACATION_RENTAL_LABEL: &str = "VacationRental";
pub struct VacationRentalIri;
impl PartialEq<&str> for VacationRentalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VACATION_RENTAL_IRI_HTTP || *other == VACATION_RENTAL_IRI_HTTPS
	}
}
impl PartialEq<VacationRentalIri> for &str {
	fn eq(&self, other: &VacationRentalIri) -> bool {
		*self == VACATION_RENTAL_IRI_HTTP || *self == VACATION_RENTAL_IRI_HTTPS
	}
}
pub struct VacationRentalIriOrLabel;
impl PartialEq<&str> for VacationRentalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VacationRentalIri || *other == VACATION_RENTAL_LABEL
	}
}
impl PartialEq<VacationRentalIriOrLabel> for &str {
	fn eq(&self, other: &VacationRentalIriOrLabel) -> bool {
		*self == VacationRentalIri || *self == VACATION_RENTAL_LABEL
	}
}
