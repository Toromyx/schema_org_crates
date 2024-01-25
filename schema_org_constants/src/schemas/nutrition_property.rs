/// <https://schema.org/nutrition>
pub const NUTRITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/nutrition";
/// <https://schema.org/nutrition>
pub const NUTRITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/nutrition";
/// <https://schema.org/nutrition>
pub const NUTRITION_PROPERTY_LABEL: &str = "nutrition";
pub struct NutritionPropertyIri;
impl PartialEq<&str> for NutritionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUTRITION_PROPERTY_IRI_HTTP || *other == NUTRITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NutritionPropertyIri> for &str {
	fn eq(&self, other: &NutritionPropertyIri) -> bool {
		*self == NUTRITION_PROPERTY_IRI_HTTP || *self == NUTRITION_PROPERTY_IRI_HTTPS
	}
}
pub struct NutritionPropertyIriOrLabel;
impl PartialEq<&str> for NutritionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NutritionPropertyIri || *other == NUTRITION_PROPERTY_LABEL
	}
}
impl PartialEq<NutritionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NutritionPropertyIriOrLabel) -> bool {
		*self == NutritionPropertyIri || *self == NUTRITION_PROPERTY_LABEL
	}
}
