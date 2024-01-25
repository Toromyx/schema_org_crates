/// <https://schema.org/LowFatDiet>
pub const LOW_FAT_DIET_IRI_HTTP: &str = "http://schema.org/LowFatDiet";
/// <https://schema.org/LowFatDiet>
pub const LOW_FAT_DIET_IRI_HTTPS: &str = "https://schema.org/LowFatDiet";
/// <https://schema.org/LowFatDiet>
pub const LOW_FAT_DIET_LABEL: &str = "LowFatDiet";
pub struct LowFatDietIri;
impl PartialEq<&str> for LowFatDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOW_FAT_DIET_IRI_HTTP || *other == LOW_FAT_DIET_IRI_HTTPS
	}
}
impl PartialEq<LowFatDietIri> for &str {
	fn eq(&self, other: &LowFatDietIri) -> bool {
		*self == LOW_FAT_DIET_IRI_HTTP || *self == LOW_FAT_DIET_IRI_HTTPS
	}
}
pub struct LowFatDietIriOrLabel;
impl PartialEq<&str> for LowFatDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LowFatDietIri || *other == LOW_FAT_DIET_LABEL
	}
}
impl PartialEq<LowFatDietIriOrLabel> for &str {
	fn eq(&self, other: &LowFatDietIriOrLabel) -> bool {
		*self == LowFatDietIri || *self == LOW_FAT_DIET_LABEL
	}
}
