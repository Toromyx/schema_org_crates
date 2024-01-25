/// <https://schema.org/ingredients>
#[deprecated = "This schema is superseded by <https://schema.org/recipeIngredient>."]
pub const INGREDIENTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/ingredients";
/// <https://schema.org/ingredients>
#[deprecated = "This schema is superseded by <https://schema.org/recipeIngredient>."]
pub const INGREDIENTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ingredients";
/// <https://schema.org/ingredients>
#[deprecated = "This schema is superseded by <https://schema.org/recipeIngredient>."]
pub const INGREDIENTS_PROPERTY_LABEL: &str = "ingredients";
pub struct IngredientsPropertyIri;
impl PartialEq<&str> for IngredientsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INGREDIENTS_PROPERTY_IRI_HTTP || *other == INGREDIENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IngredientsPropertyIri> for &str {
	fn eq(&self, other: &IngredientsPropertyIri) -> bool {
		*self == INGREDIENTS_PROPERTY_IRI_HTTP || *self == INGREDIENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct IngredientsPropertyIriOrLabel;
impl PartialEq<&str> for IngredientsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IngredientsPropertyIri || *other == INGREDIENTS_PROPERTY_LABEL
	}
}
impl PartialEq<IngredientsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IngredientsPropertyIriOrLabel) -> bool {
		*self == IngredientsPropertyIri || *self == INGREDIENTS_PROPERTY_LABEL
	}
}
