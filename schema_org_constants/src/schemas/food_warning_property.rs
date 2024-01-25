/// <https://schema.org/foodWarning>
pub const FOOD_WARNING_PROPERTY_IRI_HTTP: &str = "http://schema.org/foodWarning";
/// <https://schema.org/foodWarning>
pub const FOOD_WARNING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/foodWarning";
/// <https://schema.org/foodWarning>
pub const FOOD_WARNING_PROPERTY_LABEL: &str = "foodWarning";
pub struct FoodWarningPropertyIri;
impl PartialEq<&str> for FoodWarningPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOOD_WARNING_PROPERTY_IRI_HTTP || *other == FOOD_WARNING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FoodWarningPropertyIri> for &str {
	fn eq(&self, other: &FoodWarningPropertyIri) -> bool {
		*self == FOOD_WARNING_PROPERTY_IRI_HTTP || *self == FOOD_WARNING_PROPERTY_IRI_HTTPS
	}
}
pub struct FoodWarningPropertyIriOrLabel;
impl PartialEq<&str> for FoodWarningPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FoodWarningPropertyIri || *other == FOOD_WARNING_PROPERTY_LABEL
	}
}
impl PartialEq<FoodWarningPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FoodWarningPropertyIriOrLabel) -> bool {
		*self == FoodWarningPropertyIri || *self == FOOD_WARNING_PROPERTY_LABEL
	}
}
