/// <https://schema.org/FastFoodRestaurant>
pub const FAST_FOOD_RESTAURANT_IRI_HTTP: &str = "http://schema.org/FastFoodRestaurant";
/// <https://schema.org/FastFoodRestaurant>
pub const FAST_FOOD_RESTAURANT_IRI_HTTPS: &str = "https://schema.org/FastFoodRestaurant";
/// <https://schema.org/FastFoodRestaurant>
pub const FAST_FOOD_RESTAURANT_LABEL: &str = "FastFoodRestaurant";
pub struct FastFoodRestaurantIri;
impl PartialEq<&str> for FastFoodRestaurantIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FAST_FOOD_RESTAURANT_IRI_HTTP || *other == FAST_FOOD_RESTAURANT_IRI_HTTPS
	}
}
impl PartialEq<FastFoodRestaurantIri> for &str {
	fn eq(&self, other: &FastFoodRestaurantIri) -> bool {
		*self == FAST_FOOD_RESTAURANT_IRI_HTTP || *self == FAST_FOOD_RESTAURANT_IRI_HTTPS
	}
}
pub struct FastFoodRestaurantIriOrLabel;
impl PartialEq<&str> for FastFoodRestaurantIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FastFoodRestaurantIri || *other == FAST_FOOD_RESTAURANT_LABEL
	}
}
impl PartialEq<FastFoodRestaurantIriOrLabel> for &str {
	fn eq(&self, other: &FastFoodRestaurantIriOrLabel) -> bool {
		*self == FastFoodRestaurantIri || *self == FAST_FOOD_RESTAURANT_LABEL
	}
}
