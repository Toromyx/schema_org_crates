/// <https://schema.org/IngredientsHealthAspect>
pub const INGREDIENTS_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/IngredientsHealthAspect";
/// <https://schema.org/IngredientsHealthAspect>
pub const INGREDIENTS_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/IngredientsHealthAspect";
/// <https://schema.org/IngredientsHealthAspect>
pub const INGREDIENTS_HEALTH_ASPECT_LABEL: &str = "IngredientsHealthAspect";
pub struct IngredientsHealthAspectIri;
impl PartialEq<&str> for IngredientsHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INGREDIENTS_HEALTH_ASPECT_IRI_HTTP
			|| *other == INGREDIENTS_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<IngredientsHealthAspectIri> for &str {
	fn eq(&self, other: &IngredientsHealthAspectIri) -> bool {
		*self == INGREDIENTS_HEALTH_ASPECT_IRI_HTTP || *self == INGREDIENTS_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct IngredientsHealthAspectIriOrLabel;
impl PartialEq<&str> for IngredientsHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IngredientsHealthAspectIri || *other == INGREDIENTS_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<IngredientsHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &IngredientsHealthAspectIriOrLabel) -> bool {
		*self == IngredientsHealthAspectIri || *self == INGREDIENTS_HEALTH_ASPECT_LABEL
	}
}
