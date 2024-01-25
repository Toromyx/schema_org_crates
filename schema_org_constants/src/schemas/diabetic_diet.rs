/// <https://schema.org/DiabeticDiet>
pub const DIABETIC_DIET_IRI_HTTP: &str = "http://schema.org/DiabeticDiet";
/// <https://schema.org/DiabeticDiet>
pub const DIABETIC_DIET_IRI_HTTPS: &str = "https://schema.org/DiabeticDiet";
/// <https://schema.org/DiabeticDiet>
pub const DIABETIC_DIET_LABEL: &str = "DiabeticDiet";
pub struct DiabeticDietIri;
impl PartialEq<&str> for DiabeticDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIABETIC_DIET_IRI_HTTP || *other == DIABETIC_DIET_IRI_HTTPS
	}
}
impl PartialEq<DiabeticDietIri> for &str {
	fn eq(&self, other: &DiabeticDietIri) -> bool {
		*self == DIABETIC_DIET_IRI_HTTP || *self == DIABETIC_DIET_IRI_HTTPS
	}
}
pub struct DiabeticDietIriOrLabel;
impl PartialEq<&str> for DiabeticDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiabeticDietIri || *other == DIABETIC_DIET_LABEL
	}
}
impl PartialEq<DiabeticDietIriOrLabel> for &str {
	fn eq(&self, other: &DiabeticDietIriOrLabel) -> bool {
		*self == DiabeticDietIri || *self == DIABETIC_DIET_LABEL
	}
}
