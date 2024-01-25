/// <https://schema.org/mealService>
pub const MEAL_SERVICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/mealService";
/// <https://schema.org/mealService>
pub const MEAL_SERVICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mealService";
/// <https://schema.org/mealService>
pub const MEAL_SERVICE_PROPERTY_LABEL: &str = "mealService";
pub struct MealServicePropertyIri;
impl PartialEq<&str> for MealServicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEAL_SERVICE_PROPERTY_IRI_HTTP || *other == MEAL_SERVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MealServicePropertyIri> for &str {
	fn eq(&self, other: &MealServicePropertyIri) -> bool {
		*self == MEAL_SERVICE_PROPERTY_IRI_HTTP || *self == MEAL_SERVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct MealServicePropertyIriOrLabel;
impl PartialEq<&str> for MealServicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MealServicePropertyIri || *other == MEAL_SERVICE_PROPERTY_LABEL
	}
}
impl PartialEq<MealServicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MealServicePropertyIriOrLabel) -> bool {
		*self == MealServicePropertyIri || *self == MEAL_SERVICE_PROPERTY_LABEL
	}
}
