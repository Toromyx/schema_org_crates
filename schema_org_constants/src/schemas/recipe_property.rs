/// <https://schema.org/recipe>
pub const RECIPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/recipe";
/// <https://schema.org/recipe>
pub const RECIPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recipe";
/// <https://schema.org/recipe>
pub const RECIPE_PROPERTY_LABEL: &str = "recipe";
pub struct RecipePropertyIri;
impl PartialEq<&str> for RecipePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECIPE_PROPERTY_IRI_HTTP || *other == RECIPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecipePropertyIri> for &str {
	fn eq(&self, other: &RecipePropertyIri) -> bool {
		*self == RECIPE_PROPERTY_IRI_HTTP || *self == RECIPE_PROPERTY_IRI_HTTPS
	}
}
pub struct RecipePropertyIriOrLabel;
impl PartialEq<&str> for RecipePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecipePropertyIri || *other == RECIPE_PROPERTY_LABEL
	}
}
impl PartialEq<RecipePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecipePropertyIriOrLabel) -> bool {
		*self == RecipePropertyIri || *self == RECIPE_PROPERTY_LABEL
	}
}
