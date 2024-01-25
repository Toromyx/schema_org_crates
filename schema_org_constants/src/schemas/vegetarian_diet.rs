/// <https://schema.org/VegetarianDiet>
pub const VEGETARIAN_DIET_IRI_HTTP: &str = "http://schema.org/VegetarianDiet";
/// <https://schema.org/VegetarianDiet>
pub const VEGETARIAN_DIET_IRI_HTTPS: &str = "https://schema.org/VegetarianDiet";
/// <https://schema.org/VegetarianDiet>
pub const VEGETARIAN_DIET_LABEL: &str = "VegetarianDiet";
pub struct VegetarianDietIri;
impl PartialEq<&str> for VegetarianDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEGETARIAN_DIET_IRI_HTTP || *other == VEGETARIAN_DIET_IRI_HTTPS
	}
}
impl PartialEq<VegetarianDietIri> for &str {
	fn eq(&self, other: &VegetarianDietIri) -> bool {
		*self == VEGETARIAN_DIET_IRI_HTTP || *self == VEGETARIAN_DIET_IRI_HTTPS
	}
}
pub struct VegetarianDietIriOrLabel;
impl PartialEq<&str> for VegetarianDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VegetarianDietIri || *other == VEGETARIAN_DIET_LABEL
	}
}
impl PartialEq<VegetarianDietIriOrLabel> for &str {
	fn eq(&self, other: &VegetarianDietIriOrLabel) -> bool {
		*self == VegetarianDietIri || *self == VEGETARIAN_DIET_LABEL
	}
}
