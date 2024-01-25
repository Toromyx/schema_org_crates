/// <https://schema.org/AutoRental>
pub const AUTO_RENTAL_IRI_HTTP: &str = "http://schema.org/AutoRental";
/// <https://schema.org/AutoRental>
pub const AUTO_RENTAL_IRI_HTTPS: &str = "https://schema.org/AutoRental";
/// <https://schema.org/AutoRental>
pub const AUTO_RENTAL_LABEL: &str = "AutoRental";
pub struct AutoRentalIri;
impl PartialEq<&str> for AutoRentalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTO_RENTAL_IRI_HTTP || *other == AUTO_RENTAL_IRI_HTTPS
	}
}
impl PartialEq<AutoRentalIri> for &str {
	fn eq(&self, other: &AutoRentalIri) -> bool {
		*self == AUTO_RENTAL_IRI_HTTP || *self == AUTO_RENTAL_IRI_HTTPS
	}
}
pub struct AutoRentalIriOrLabel;
impl PartialEq<&str> for AutoRentalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AutoRentalIri || *other == AUTO_RENTAL_LABEL
	}
}
impl PartialEq<AutoRentalIriOrLabel> for &str {
	fn eq(&self, other: &AutoRentalIriOrLabel) -> bool {
		*self == AutoRentalIri || *self == AUTO_RENTAL_LABEL
	}
}
