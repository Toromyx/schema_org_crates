/// <https://schema.org/LowCalorieDiet>
pub const LOW_CALORIE_DIET_IRI_HTTP: &str = "http://schema.org/LowCalorieDiet";
/// <https://schema.org/LowCalorieDiet>
pub const LOW_CALORIE_DIET_IRI_HTTPS: &str = "https://schema.org/LowCalorieDiet";
/// <https://schema.org/LowCalorieDiet>
pub const LOW_CALORIE_DIET_LABEL: &str = "LowCalorieDiet";
pub struct LowCalorieDietIri;
impl PartialEq<&str> for LowCalorieDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOW_CALORIE_DIET_IRI_HTTP || *other == LOW_CALORIE_DIET_IRI_HTTPS
	}
}
impl PartialEq<LowCalorieDietIri> for &str {
	fn eq(&self, other: &LowCalorieDietIri) -> bool {
		*self == LOW_CALORIE_DIET_IRI_HTTP || *self == LOW_CALORIE_DIET_IRI_HTTPS
	}
}
pub struct LowCalorieDietIriOrLabel;
impl PartialEq<&str> for LowCalorieDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LowCalorieDietIri || *other == LOW_CALORIE_DIET_LABEL
	}
}
impl PartialEq<LowCalorieDietIriOrLabel> for &str {
	fn eq(&self, other: &LowCalorieDietIriOrLabel) -> bool {
		*self == LowCalorieDietIri || *self == LOW_CALORIE_DIET_LABEL
	}
}
