/// <https://schema.org/HinduDiet>
pub const HINDU_DIET_IRI_HTTP: &str = "http://schema.org/HinduDiet";
/// <https://schema.org/HinduDiet>
pub const HINDU_DIET_IRI_HTTPS: &str = "https://schema.org/HinduDiet";
/// <https://schema.org/HinduDiet>
pub const HINDU_DIET_LABEL: &str = "HinduDiet";
pub struct HinduDietIri;
impl PartialEq<&str> for HinduDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HINDU_DIET_IRI_HTTP || *other == HINDU_DIET_IRI_HTTPS
	}
}
impl PartialEq<HinduDietIri> for &str {
	fn eq(&self, other: &HinduDietIri) -> bool {
		*self == HINDU_DIET_IRI_HTTP || *self == HINDU_DIET_IRI_HTTPS
	}
}
pub struct HinduDietIriOrLabel;
impl PartialEq<&str> for HinduDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HinduDietIri || *other == HINDU_DIET_LABEL
	}
}
impl PartialEq<HinduDietIriOrLabel> for &str {
	fn eq(&self, other: &HinduDietIriOrLabel) -> bool {
		*self == HinduDietIri || *self == HINDU_DIET_LABEL
	}
}
