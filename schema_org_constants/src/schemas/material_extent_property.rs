/// <https://schema.org/materialExtent>
pub const MATERIAL_EXTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/materialExtent";
/// <https://schema.org/materialExtent>
pub const MATERIAL_EXTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/materialExtent";
/// <https://schema.org/materialExtent>
pub const MATERIAL_EXTENT_PROPERTY_LABEL: &str = "materialExtent";
pub struct MaterialExtentPropertyIri;
impl PartialEq<&str> for MaterialExtentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MATERIAL_EXTENT_PROPERTY_IRI_HTTP || *other == MATERIAL_EXTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaterialExtentPropertyIri> for &str {
	fn eq(&self, other: &MaterialExtentPropertyIri) -> bool {
		*self == MATERIAL_EXTENT_PROPERTY_IRI_HTTP || *self == MATERIAL_EXTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct MaterialExtentPropertyIriOrLabel;
impl PartialEq<&str> for MaterialExtentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaterialExtentPropertyIri || *other == MATERIAL_EXTENT_PROPERTY_LABEL
	}
}
impl PartialEq<MaterialExtentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaterialExtentPropertyIriOrLabel) -> bool {
		*self == MaterialExtentPropertyIri || *self == MATERIAL_EXTENT_PROPERTY_LABEL
	}
}
