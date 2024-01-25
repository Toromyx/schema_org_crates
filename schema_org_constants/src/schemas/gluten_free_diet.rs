/// <https://schema.org/GlutenFreeDiet>
pub const GLUTEN_FREE_DIET_IRI_HTTP: &str = "http://schema.org/GlutenFreeDiet";
/// <https://schema.org/GlutenFreeDiet>
pub const GLUTEN_FREE_DIET_IRI_HTTPS: &str = "https://schema.org/GlutenFreeDiet";
/// <https://schema.org/GlutenFreeDiet>
pub const GLUTEN_FREE_DIET_LABEL: &str = "GlutenFreeDiet";
pub struct GlutenFreeDietIri;
impl PartialEq<&str> for GlutenFreeDietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GLUTEN_FREE_DIET_IRI_HTTP || *other == GLUTEN_FREE_DIET_IRI_HTTPS
	}
}
impl PartialEq<GlutenFreeDietIri> for &str {
	fn eq(&self, other: &GlutenFreeDietIri) -> bool {
		*self == GLUTEN_FREE_DIET_IRI_HTTP || *self == GLUTEN_FREE_DIET_IRI_HTTPS
	}
}
pub struct GlutenFreeDietIriOrLabel;
impl PartialEq<&str> for GlutenFreeDietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GlutenFreeDietIri || *other == GLUTEN_FREE_DIET_LABEL
	}
}
impl PartialEq<GlutenFreeDietIriOrLabel> for &str {
	fn eq(&self, other: &GlutenFreeDietIriOrLabel) -> bool {
		*self == GlutenFreeDietIri || *self == GLUTEN_FREE_DIET_LABEL
	}
}
