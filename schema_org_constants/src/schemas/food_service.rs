/// <https://schema.org/FoodService>
pub const FOOD_SERVICE_IRI_HTTP: &str = "http://schema.org/FoodService";
/// <https://schema.org/FoodService>
pub const FOOD_SERVICE_IRI_HTTPS: &str = "https://schema.org/FoodService";
/// <https://schema.org/FoodService>
pub const FOOD_SERVICE_LABEL: &str = "FoodService";
pub struct FoodServiceIri;
impl PartialEq<&str> for FoodServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOOD_SERVICE_IRI_HTTP || *other == FOOD_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<FoodServiceIri> for &str {
	fn eq(&self, other: &FoodServiceIri) -> bool {
		*self == FOOD_SERVICE_IRI_HTTP || *self == FOOD_SERVICE_IRI_HTTPS
	}
}
pub struct FoodServiceIriOrLabel;
impl PartialEq<&str> for FoodServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoodServiceIri || *other == FOOD_SERVICE_LABEL
	}
}
impl PartialEq<FoodServiceIriOrLabel> for &str {
	fn eq(&self, other: &FoodServiceIriOrLabel) -> bool {
		*self == FoodServiceIri || *self == FOOD_SERVICE_LABEL
	}
}
