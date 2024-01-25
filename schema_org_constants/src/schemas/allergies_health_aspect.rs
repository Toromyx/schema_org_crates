/// <https://schema.org/AllergiesHealthAspect>
pub const ALLERGIES_HEALTH_ASPECT_IRI_HTTP: &str = "http://schema.org/AllergiesHealthAspect";
/// <https://schema.org/AllergiesHealthAspect>
pub const ALLERGIES_HEALTH_ASPECT_IRI_HTTPS: &str = "https://schema.org/AllergiesHealthAspect";
/// <https://schema.org/AllergiesHealthAspect>
pub const ALLERGIES_HEALTH_ASPECT_LABEL: &str = "AllergiesHealthAspect";
pub struct AllergiesHealthAspectIri;
impl PartialEq<&str> for AllergiesHealthAspectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALLERGIES_HEALTH_ASPECT_IRI_HTTP || *other == ALLERGIES_HEALTH_ASPECT_IRI_HTTPS
	}
}
impl PartialEq<AllergiesHealthAspectIri> for &str {
	fn eq(&self, other: &AllergiesHealthAspectIri) -> bool {
		*self == ALLERGIES_HEALTH_ASPECT_IRI_HTTP || *self == ALLERGIES_HEALTH_ASPECT_IRI_HTTPS
	}
}
pub struct AllergiesHealthAspectIriOrLabel;
impl PartialEq<&str> for AllergiesHealthAspectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AllergiesHealthAspectIri || *other == ALLERGIES_HEALTH_ASPECT_LABEL
	}
}
impl PartialEq<AllergiesHealthAspectIriOrLabel> for &str {
	fn eq(&self, other: &AllergiesHealthAspectIriOrLabel) -> bool {
		*self == AllergiesHealthAspectIri || *self == ALLERGIES_HEALTH_ASPECT_LABEL
	}
}
