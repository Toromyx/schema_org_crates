/// <https://schema.org/activeIngredient>
pub const ACTIVE_INGREDIENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/activeIngredient";
/// <https://schema.org/activeIngredient>
pub const ACTIVE_INGREDIENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/activeIngredient";
/// <https://schema.org/activeIngredient>
pub const ACTIVE_INGREDIENT_PROPERTY_LABEL: &str = "activeIngredient";
pub struct ActiveIngredientPropertyIri;
impl PartialEq<&str> for ActiveIngredientPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTIVE_INGREDIENT_PROPERTY_IRI_HTTP
			|| *other == ACTIVE_INGREDIENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActiveIngredientPropertyIri> for &str {
	fn eq(&self, other: &ActiveIngredientPropertyIri) -> bool {
		*self == ACTIVE_INGREDIENT_PROPERTY_IRI_HTTP
			|| *self == ACTIVE_INGREDIENT_PROPERTY_IRI_HTTPS
	}
}
pub struct ActiveIngredientPropertyIriOrLabel;
impl PartialEq<&str> for ActiveIngredientPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActiveIngredientPropertyIri || *other == ACTIVE_INGREDIENT_PROPERTY_LABEL
	}
}
impl PartialEq<ActiveIngredientPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActiveIngredientPropertyIriOrLabel) -> bool {
		*self == ActiveIngredientPropertyIri || *self == ACTIVE_INGREDIENT_PROPERTY_LABEL
	}
}
