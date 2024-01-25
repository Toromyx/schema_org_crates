/// <https://schema.org/recipeCuisine>
pub const RECIPE_CUISINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/recipeCuisine";
/// <https://schema.org/recipeCuisine>
pub const RECIPE_CUISINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recipeCuisine";
/// <https://schema.org/recipeCuisine>
pub const RECIPE_CUISINE_PROPERTY_LABEL: &str = "recipeCuisine";
pub struct RecipeCuisinePropertyIri;
impl PartialEq<&str> for RecipeCuisinePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECIPE_CUISINE_PROPERTY_IRI_HTTP || *other == RECIPE_CUISINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecipeCuisinePropertyIri> for &str {
	fn eq(&self, other: &RecipeCuisinePropertyIri) -> bool {
		*self == RECIPE_CUISINE_PROPERTY_IRI_HTTP || *self == RECIPE_CUISINE_PROPERTY_IRI_HTTPS
	}
}
pub struct RecipeCuisinePropertyIriOrLabel;
impl PartialEq<&str> for RecipeCuisinePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecipeCuisinePropertyIri || *other == RECIPE_CUISINE_PROPERTY_LABEL
	}
}
impl PartialEq<RecipeCuisinePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecipeCuisinePropertyIriOrLabel) -> bool {
		*self == RecipeCuisinePropertyIri || *self == RECIPE_CUISINE_PROPERTY_LABEL
	}
}
