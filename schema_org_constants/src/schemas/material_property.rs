/// <https://schema.org/material>
pub const MATERIAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/material";
/// <https://schema.org/material>
pub const MATERIAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/material";
/// <https://schema.org/material>
pub const MATERIAL_PROPERTY_LABEL: &str = "material";
pub struct MaterialPropertyIri;
impl PartialEq<&str> for MaterialPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MATERIAL_PROPERTY_IRI_HTTP || *other == MATERIAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaterialPropertyIri> for &str {
	fn eq(&self, other: &MaterialPropertyIri) -> bool {
		*self == MATERIAL_PROPERTY_IRI_HTTP || *self == MATERIAL_PROPERTY_IRI_HTTPS
	}
}
pub struct MaterialPropertyIriOrLabel;
impl PartialEq<&str> for MaterialPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaterialPropertyIri || *other == MATERIAL_PROPERTY_LABEL
	}
}
impl PartialEq<MaterialPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaterialPropertyIriOrLabel) -> bool {
		*self == MaterialPropertyIri || *self == MATERIAL_PROPERTY_LABEL
	}
}
