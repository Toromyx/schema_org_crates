/// <https://schema.org/FoodEstablishment>
pub const FOOD_ESTABLISHMENT_IRI_HTTP: &str = "http://schema.org/FoodEstablishment";
/// <https://schema.org/FoodEstablishment>
pub const FOOD_ESTABLISHMENT_IRI_HTTPS: &str = "https://schema.org/FoodEstablishment";
/// <https://schema.org/FoodEstablishment>
pub const FOOD_ESTABLISHMENT_LABEL: &str = "FoodEstablishment";
pub struct FoodEstablishmentIri;
impl PartialEq<&str> for FoodEstablishmentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOOD_ESTABLISHMENT_IRI_HTTP || *other == FOOD_ESTABLISHMENT_IRI_HTTPS
	}
}
impl PartialEq<FoodEstablishmentIri> for &str {
	fn eq(&self, other: &FoodEstablishmentIri) -> bool {
		*self == FOOD_ESTABLISHMENT_IRI_HTTP || *self == FOOD_ESTABLISHMENT_IRI_HTTPS
	}
}
pub struct FoodEstablishmentIriOrLabel;
impl PartialEq<&str> for FoodEstablishmentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoodEstablishmentIri || *other == FOOD_ESTABLISHMENT_LABEL
	}
}
impl PartialEq<FoodEstablishmentIriOrLabel> for &str {
	fn eq(&self, other: &FoodEstablishmentIriOrLabel) -> bool {
		*self == FoodEstablishmentIri || *self == FOOD_ESTABLISHMENT_LABEL
	}
}
