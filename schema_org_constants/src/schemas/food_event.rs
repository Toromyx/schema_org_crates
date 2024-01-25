/// <https://schema.org/FoodEvent>
pub const FOOD_EVENT_IRI_HTTP: &str = "http://schema.org/FoodEvent";
/// <https://schema.org/FoodEvent>
pub const FOOD_EVENT_IRI_HTTPS: &str = "https://schema.org/FoodEvent";
/// <https://schema.org/FoodEvent>
pub const FOOD_EVENT_LABEL: &str = "FoodEvent";
pub struct FoodEventIri;
impl PartialEq<&str> for FoodEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOOD_EVENT_IRI_HTTP || *other == FOOD_EVENT_IRI_HTTPS
	}
}
impl PartialEq<FoodEventIri> for &str {
	fn eq(&self, other: &FoodEventIri) -> bool {
		*self == FOOD_EVENT_IRI_HTTP || *self == FOOD_EVENT_IRI_HTTPS
	}
}
pub struct FoodEventIriOrLabel;
impl PartialEq<&str> for FoodEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoodEventIri || *other == FOOD_EVENT_LABEL
	}
}
impl PartialEq<FoodEventIriOrLabel> for &str {
	fn eq(&self, other: &FoodEventIriOrLabel) -> bool {
		*self == FoodEventIri || *self == FOOD_EVENT_LABEL
	}
}
