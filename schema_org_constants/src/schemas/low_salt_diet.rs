/// <https://schema.org/LowSaltDiet>
pub const LOW_SALT_DIET_IRI_HTTP: &str = "http://schema.org/LowSaltDiet";
/// <https://schema.org/LowSaltDiet>
pub const LOW_SALT_DIET_IRI_HTTPS: &str = "https://schema.org/LowSaltDiet";
/// <https://schema.org/LowSaltDiet>
pub const LOW_SALT_DIET_LABEL: &str = "LowSaltDiet";
pub struct LowSaltDietIri;
impl PartialEq<&str> for LowSaltDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOW_SALT_DIET_IRI_HTTP || *other == LOW_SALT_DIET_IRI_HTTPS
	}
}
impl PartialEq<LowSaltDietIri> for &str {
	fn eq(&self, other: &LowSaltDietIri) -> bool {
		*self == LOW_SALT_DIET_IRI_HTTP || *self == LOW_SALT_DIET_IRI_HTTPS
	}
}
pub struct LowSaltDietIriOrLabel;
impl PartialEq<&str> for LowSaltDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LowSaltDietIri || *other == LOW_SALT_DIET_LABEL
	}
}
impl PartialEq<LowSaltDietIriOrLabel> for &str {
	fn eq(&self, other: &LowSaltDietIriOrLabel) -> bool {
		*self == LowSaltDietIri || *self == LOW_SALT_DIET_LABEL
	}
}
