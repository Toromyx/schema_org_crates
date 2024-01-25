/// <https://schema.org/foodEstablishment>
pub const FOOD_ESTABLISHMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/foodEstablishment";
/// <https://schema.org/foodEstablishment>
pub const FOOD_ESTABLISHMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/foodEstablishment";
/// <https://schema.org/foodEstablishment>
pub const FOOD_ESTABLISHMENT_PROPERTY_LABEL: &str = "foodEstablishment";
pub struct FoodEstablishmentPropertyIri;
impl PartialEq<&str> for FoodEstablishmentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOOD_ESTABLISHMENT_PROPERTY_IRI_HTTP
			|| *other == FOOD_ESTABLISHMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FoodEstablishmentPropertyIri> for &str {
	fn eq(&self, other: &FoodEstablishmentPropertyIri) -> bool {
		*self == FOOD_ESTABLISHMENT_PROPERTY_IRI_HTTP
			|| *self == FOOD_ESTABLISHMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct FoodEstablishmentPropertyIriOrLabel;
impl PartialEq<&str> for FoodEstablishmentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoodEstablishmentPropertyIri || *other == FOOD_ESTABLISHMENT_PROPERTY_LABEL
	}
}
impl PartialEq<FoodEstablishmentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FoodEstablishmentPropertyIriOrLabel) -> bool {
		*self == FoodEstablishmentPropertyIri || *self == FOOD_ESTABLISHMENT_PROPERTY_LABEL
	}
}
