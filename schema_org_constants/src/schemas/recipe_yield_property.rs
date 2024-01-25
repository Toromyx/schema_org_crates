/// <https://schema.org/recipeYield>
pub const RECIPE_YIELD_PROPERTY_IRI_HTTP: &str = "http://schema.org/recipeYield";
/// <https://schema.org/recipeYield>
pub const RECIPE_YIELD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recipeYield";
/// <https://schema.org/recipeYield>
pub const RECIPE_YIELD_PROPERTY_LABEL: &str = "recipeYield";
pub struct RecipeYieldPropertyIri;
impl PartialEq<&str> for RecipeYieldPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECIPE_YIELD_PROPERTY_IRI_HTTP || *other == RECIPE_YIELD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecipeYieldPropertyIri> for &str {
	fn eq(&self, other: &RecipeYieldPropertyIri) -> bool {
		*self == RECIPE_YIELD_PROPERTY_IRI_HTTP || *self == RECIPE_YIELD_PROPERTY_IRI_HTTPS
	}
}
pub struct RecipeYieldPropertyIriOrLabel;
impl PartialEq<&str> for RecipeYieldPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecipeYieldPropertyIri || *other == RECIPE_YIELD_PROPERTY_LABEL
	}
}
impl PartialEq<RecipeYieldPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecipeYieldPropertyIriOrLabel) -> bool {
		*self == RecipeYieldPropertyIri || *self == RECIPE_YIELD_PROPERTY_LABEL
	}
}
