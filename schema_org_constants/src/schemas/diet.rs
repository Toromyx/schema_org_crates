/// <https://schema.org/Diet>
pub const DIET_IRI_HTTP: &str = "http://schema.org/Diet";
/// <https://schema.org/Diet>
pub const DIET_IRI_HTTPS: &str = "https://schema.org/Diet";
/// <https://schema.org/Diet>
pub const DIET_LABEL: &str = "Diet";
pub struct DietIri;
impl PartialEq<&str> for DietIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIET_IRI_HTTP || *other == DIET_IRI_HTTPS
	}
}
impl PartialEq<DietIri> for &str {
	fn eq(&self, other: &DietIri) -> bool {
		*self == DIET_IRI_HTTP || *self == DIET_IRI_HTTPS
	}
}
pub struct DietIriOrLabel;
impl PartialEq<&str> for DietIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DietIri || *other == DIET_LABEL
	}
}
impl PartialEq<DietIriOrLabel> for &str {
	fn eq(&self, other: &DietIriOrLabel) -> bool {
		*self == DietIri || *self == DIET_LABEL
	}
}
