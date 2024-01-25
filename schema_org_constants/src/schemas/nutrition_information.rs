/// <https://schema.org/NutritionInformation>
pub const NUTRITION_INFORMATION_IRI_HTTP: &str = "http://schema.org/NutritionInformation";
/// <https://schema.org/NutritionInformation>
pub const NUTRITION_INFORMATION_IRI_HTTPS: &str = "https://schema.org/NutritionInformation";
/// <https://schema.org/NutritionInformation>
pub const NUTRITION_INFORMATION_LABEL: &str = "NutritionInformation";
pub struct NutritionInformationIri;
impl PartialEq<&str> for NutritionInformationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUTRITION_INFORMATION_IRI_HTTP || *other == NUTRITION_INFORMATION_IRI_HTTPS
	}
}
impl PartialEq<NutritionInformationIri> for &str {
	fn eq(&self, other: &NutritionInformationIri) -> bool {
		*self == NUTRITION_INFORMATION_IRI_HTTP || *self == NUTRITION_INFORMATION_IRI_HTTPS
	}
}
pub struct NutritionInformationIriOrLabel;
impl PartialEq<&str> for NutritionInformationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NutritionInformationIri || *other == NUTRITION_INFORMATION_LABEL
	}
}
impl PartialEq<NutritionInformationIriOrLabel> for &str {
	fn eq(&self, other: &NutritionInformationIriOrLabel) -> bool {
		*self == NutritionInformationIri || *self == NUTRITION_INFORMATION_LABEL
	}
}
