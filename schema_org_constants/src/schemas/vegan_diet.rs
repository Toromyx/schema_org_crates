/// <https://schema.org/VeganDiet>
pub const VEGAN_DIET_IRI_HTTP: &str = "http://schema.org/VeganDiet";
/// <https://schema.org/VeganDiet>
pub const VEGAN_DIET_IRI_HTTPS: &str = "https://schema.org/VeganDiet";
/// <https://schema.org/VeganDiet>
pub const VEGAN_DIET_LABEL: &str = "VeganDiet";
pub struct VeganDietIri;
impl PartialEq<&str> for VeganDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEGAN_DIET_IRI_HTTP || *other == VEGAN_DIET_IRI_HTTPS
	}
}
impl PartialEq<VeganDietIri> for &str {
	fn eq(&self, other: &VeganDietIri) -> bool {
		*self == VEGAN_DIET_IRI_HTTP || *self == VEGAN_DIET_IRI_HTTPS
	}
}
pub struct VeganDietIriOrLabel;
impl PartialEq<&str> for VeganDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VeganDietIri || *other == VEGAN_DIET_LABEL
	}
}
impl PartialEq<VeganDietIriOrLabel> for &str {
	fn eq(&self, other: &VeganDietIriOrLabel) -> bool {
		*self == VeganDietIri || *self == VEGAN_DIET_LABEL
	}
}
