/// <https://schema.org/DietNutrition>
pub const DIET_NUTRITION_IRI_HTTP: &str = "http://schema.org/DietNutrition";
/// <https://schema.org/DietNutrition>
pub const DIET_NUTRITION_IRI_HTTPS: &str = "https://schema.org/DietNutrition";
/// <https://schema.org/DietNutrition>
pub const DIET_NUTRITION_LABEL: &str = "DietNutrition";
pub struct DietNutritionIri;
impl PartialEq<&str> for DietNutritionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIET_NUTRITION_IRI_HTTP || *other == DIET_NUTRITION_IRI_HTTPS
	}
}
impl PartialEq<DietNutritionIri> for &str {
	fn eq(&self, other: &DietNutritionIri) -> bool {
		*self == DIET_NUTRITION_IRI_HTTP || *self == DIET_NUTRITION_IRI_HTTPS
	}
}
pub struct DietNutritionIriOrLabel;
impl PartialEq<&str> for DietNutritionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DietNutritionIri || *other == DIET_NUTRITION_LABEL
	}
}
impl PartialEq<DietNutritionIriOrLabel> for &str {
	fn eq(&self, other: &DietNutritionIriOrLabel) -> bool {
		*self == DietNutritionIri || *self == DIET_NUTRITION_LABEL
	}
}
