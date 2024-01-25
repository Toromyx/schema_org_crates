/// <https://schema.org/HalalDiet>
pub const HALAL_DIET_IRI_HTTP: &str = "http://schema.org/HalalDiet";
/// <https://schema.org/HalalDiet>
pub const HALAL_DIET_IRI_HTTPS: &str = "https://schema.org/HalalDiet";
/// <https://schema.org/HalalDiet>
pub const HALAL_DIET_LABEL: &str = "HalalDiet";
pub struct HalalDietIri;
impl PartialEq<&str> for HalalDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HALAL_DIET_IRI_HTTP || *other == HALAL_DIET_IRI_HTTPS
	}
}
impl PartialEq<HalalDietIri> for &str {
	fn eq(&self, other: &HalalDietIri) -> bool {
		*self == HALAL_DIET_IRI_HTTP || *self == HALAL_DIET_IRI_HTTPS
	}
}
pub struct HalalDietIriOrLabel;
impl PartialEq<&str> for HalalDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HalalDietIri || *other == HALAL_DIET_LABEL
	}
}
impl PartialEq<HalalDietIriOrLabel> for &str {
	fn eq(&self, other: &HalalDietIriOrLabel) -> bool {
		*self == HalalDietIri || *self == HALAL_DIET_LABEL
	}
}
