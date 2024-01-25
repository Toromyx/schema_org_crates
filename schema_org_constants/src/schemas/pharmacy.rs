/// <https://schema.org/Pharmacy>
pub const PHARMACY_IRI_HTTP: &str = "http://schema.org/Pharmacy";
/// <https://schema.org/Pharmacy>
pub const PHARMACY_IRI_HTTPS: &str = "https://schema.org/Pharmacy";
/// <https://schema.org/Pharmacy>
pub const PHARMACY_LABEL: &str = "Pharmacy";
pub struct PharmacyIri;
impl PartialEq<&str> for PharmacyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHARMACY_IRI_HTTP || *other == PHARMACY_IRI_HTTPS
	}
}
impl PartialEq<PharmacyIri> for &str {
	fn eq(&self, other: &PharmacyIri) -> bool {
		*self == PHARMACY_IRI_HTTP || *self == PHARMACY_IRI_HTTPS
	}
}
pub struct PharmacyIriOrLabel;
impl PartialEq<&str> for PharmacyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PharmacyIri || *other == PHARMACY_LABEL
	}
}
impl PartialEq<PharmacyIriOrLabel> for &str {
	fn eq(&self, other: &PharmacyIriOrLabel) -> bool {
		*self == PharmacyIri || *self == PHARMACY_LABEL
	}
}
