/// <https://schema.org/KosherDiet>
pub const KOSHER_DIET_IRI_HTTP: &str = "http://schema.org/KosherDiet";
/// <https://schema.org/KosherDiet>
pub const KOSHER_DIET_IRI_HTTPS: &str = "https://schema.org/KosherDiet";
/// <https://schema.org/KosherDiet>
pub const KOSHER_DIET_LABEL: &str = "KosherDiet";
pub struct KosherDietIri;
impl PartialEq<&str> for KosherDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == KOSHER_DIET_IRI_HTTP || *other == KOSHER_DIET_IRI_HTTPS
	}
}
impl PartialEq<KosherDietIri> for &str {
	fn eq(&self, other: &KosherDietIri) -> bool {
		*self == KOSHER_DIET_IRI_HTTP || *self == KOSHER_DIET_IRI_HTTPS
	}
}
pub struct KosherDietIriOrLabel;
impl PartialEq<&str> for KosherDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == KosherDietIri || *other == KOSHER_DIET_LABEL
	}
}
impl PartialEq<KosherDietIriOrLabel> for &str {
	fn eq(&self, other: &KosherDietIriOrLabel) -> bool {
		*self == KosherDietIri || *self == KOSHER_DIET_LABEL
	}
}
