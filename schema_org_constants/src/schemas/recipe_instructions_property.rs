/// <https://schema.org/recipeInstructions>
pub const RECIPE_INSTRUCTIONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/recipeInstructions";
/// <https://schema.org/recipeInstructions>
pub const RECIPE_INSTRUCTIONS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recipeInstructions";
/// <https://schema.org/recipeInstructions>
pub const RECIPE_INSTRUCTIONS_PROPERTY_LABEL: &str = "recipeInstructions";
pub struct RecipeInstructionsPropertyIri;
impl PartialEq<&str> for RecipeInstructionsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECIPE_INSTRUCTIONS_PROPERTY_IRI_HTTP
			|| *other == RECIPE_INSTRUCTIONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecipeInstructionsPropertyIri> for &str {
	fn eq(&self, other: &RecipeInstructionsPropertyIri) -> bool {
		*self == RECIPE_INSTRUCTIONS_PROPERTY_IRI_HTTP
			|| *self == RECIPE_INSTRUCTIONS_PROPERTY_IRI_HTTPS
	}
}
pub struct RecipeInstructionsPropertyIriOrLabel;
impl PartialEq<&str> for RecipeInstructionsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecipeInstructionsPropertyIri || *other == RECIPE_INSTRUCTIONS_PROPERTY_LABEL
	}
}
impl PartialEq<RecipeInstructionsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecipeInstructionsPropertyIriOrLabel) -> bool {
		*self == RecipeInstructionsPropertyIri || *self == RECIPE_INSTRUCTIONS_PROPERTY_LABEL
	}
}
