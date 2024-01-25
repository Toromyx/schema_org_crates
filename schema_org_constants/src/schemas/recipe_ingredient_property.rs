/// <https://schema.org/recipeIngredient>
pub const RECIPE_INGREDIENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/recipeIngredient";
/// <https://schema.org/recipeIngredient>
pub const RECIPE_INGREDIENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recipeIngredient";
/// <https://schema.org/recipeIngredient>
pub const RECIPE_INGREDIENT_PROPERTY_LABEL: &str = "recipeIngredient";
pub struct RecipeIngredientPropertyIri;
impl PartialEq<&str> for RecipeIngredientPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECIPE_INGREDIENT_PROPERTY_IRI_HTTP
			|| *other == RECIPE_INGREDIENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecipeIngredientPropertyIri> for &str {
	fn eq(&self, other: &RecipeIngredientPropertyIri) -> bool {
		*self == RECIPE_INGREDIENT_PROPERTY_IRI_HTTP
			|| *self == RECIPE_INGREDIENT_PROPERTY_IRI_HTTPS
	}
}
pub struct RecipeIngredientPropertyIriOrLabel;
impl PartialEq<&str> for RecipeIngredientPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecipeIngredientPropertyIri || *other == RECIPE_INGREDIENT_PROPERTY_LABEL
	}
}
impl PartialEq<RecipeIngredientPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecipeIngredientPropertyIriOrLabel) -> bool {
		*self == RecipeIngredientPropertyIri || *self == RECIPE_INGREDIENT_PROPERTY_LABEL
	}
}
