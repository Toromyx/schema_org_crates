/// <https://schema.org/recipeCategory>
pub const RECIPE_CATEGORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/recipeCategory";
/// <https://schema.org/recipeCategory>
pub const RECIPE_CATEGORY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recipeCategory";
/// <https://schema.org/recipeCategory>
pub const RECIPE_CATEGORY_PROPERTY_LABEL: &str = "recipeCategory";
pub struct RecipeCategoryPropertyIri;
impl PartialEq<&str> for RecipeCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECIPE_CATEGORY_PROPERTY_IRI_HTTP || *other == RECIPE_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecipeCategoryPropertyIri> for &str {
	fn eq(&self, other: &RecipeCategoryPropertyIri) -> bool {
		*self == RECIPE_CATEGORY_PROPERTY_IRI_HTTP || *self == RECIPE_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct RecipeCategoryPropertyIriOrLabel;
impl PartialEq<&str> for RecipeCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecipeCategoryPropertyIri || *other == RECIPE_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<RecipeCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecipeCategoryPropertyIriOrLabel) -> bool {
		*self == RecipeCategoryPropertyIri || *self == RECIPE_CATEGORY_PROPERTY_LABEL
	}
}
