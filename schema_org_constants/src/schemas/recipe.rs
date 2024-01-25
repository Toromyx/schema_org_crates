/// <https://schema.org/Recipe>
pub const RECIPE_IRI_HTTP: &str = "http://schema.org/Recipe";
/// <https://schema.org/Recipe>
pub const RECIPE_IRI_HTTPS: &str = "https://schema.org/Recipe";
/// <https://schema.org/Recipe>
pub const RECIPE_LABEL: &str = "Recipe";
pub struct RecipeIri;
impl PartialEq<&str> for RecipeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECIPE_IRI_HTTP || *other == RECIPE_IRI_HTTPS
	}
}
impl PartialEq<RecipeIri> for &str {
	fn eq(&self, other: &RecipeIri) -> bool {
		*self == RECIPE_IRI_HTTP || *self == RECIPE_IRI_HTTPS
	}
}
pub struct RecipeIriOrLabel;
impl PartialEq<&str> for RecipeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecipeIri || *other == RECIPE_LABEL
	}
}
impl PartialEq<RecipeIriOrLabel> for &str {
	fn eq(&self, other: &RecipeIriOrLabel) -> bool {
		*self == RecipeIri || *self == RECIPE_LABEL
	}
}
