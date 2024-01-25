/// <https://schema.org/foodEvent>
pub const FOOD_EVENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/foodEvent";
/// <https://schema.org/foodEvent>
pub const FOOD_EVENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/foodEvent";
/// <https://schema.org/foodEvent>
pub const FOOD_EVENT_PROPERTY_LABEL: &str = "foodEvent";
pub struct FoodEventPropertyIri;
impl PartialEq<&str> for FoodEventPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOOD_EVENT_PROPERTY_IRI_HTTP || *other == FOOD_EVENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FoodEventPropertyIri> for &str {
	fn eq(&self, other: &FoodEventPropertyIri) -> bool {
		*self == FOOD_EVENT_PROPERTY_IRI_HTTP || *self == FOOD_EVENT_PROPERTY_IRI_HTTPS
	}
}
pub struct FoodEventPropertyIriOrLabel;
impl PartialEq<&str> for FoodEventPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoodEventPropertyIri || *other == FOOD_EVENT_PROPERTY_LABEL
	}
}
impl PartialEq<FoodEventPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FoodEventPropertyIriOrLabel) -> bool {
		*self == FoodEventPropertyIri || *self == FOOD_EVENT_PROPERTY_LABEL
	}
}
