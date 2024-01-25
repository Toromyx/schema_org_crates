/// <https://schema.org/diet>
pub const DIET_PROPERTY_IRI_HTTP: &str = "http://schema.org/diet";
/// <https://schema.org/diet>
pub const DIET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/diet";
/// <https://schema.org/diet>
pub const DIET_PROPERTY_LABEL: &str = "diet";
pub struct DietPropertyIri;
impl PartialEq<&str> for DietPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIET_PROPERTY_IRI_HTTP || *other == DIET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DietPropertyIri> for &str {
	fn eq(&self, other: &DietPropertyIri) -> bool {
		*self == DIET_PROPERTY_IRI_HTTP || *self == DIET_PROPERTY_IRI_HTTPS
	}
}
pub struct DietPropertyIriOrLabel;
impl PartialEq<&str> for DietPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DietPropertyIri || *other == DIET_PROPERTY_LABEL
	}
}
impl PartialEq<DietPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DietPropertyIriOrLabel) -> bool {
		*self == DietPropertyIri || *self == DIET_PROPERTY_LABEL
	}
}
