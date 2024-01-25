/// <https://schema.org/roofLoad>
pub const ROOF_LOAD_PROPERTY_IRI_HTTP: &str = "http://schema.org/roofLoad";
/// <https://schema.org/roofLoad>
pub const ROOF_LOAD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/roofLoad";
/// <https://schema.org/roofLoad>
pub const ROOF_LOAD_PROPERTY_LABEL: &str = "roofLoad";
pub struct RoofLoadPropertyIri;
impl PartialEq<&str> for RoofLoadPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ROOF_LOAD_PROPERTY_IRI_HTTP || *other == ROOF_LOAD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RoofLoadPropertyIri> for &str {
	fn eq(&self, other: &RoofLoadPropertyIri) -> bool {
		*self == ROOF_LOAD_PROPERTY_IRI_HTTP || *self == ROOF_LOAD_PROPERTY_IRI_HTTPS
	}
}
pub struct RoofLoadPropertyIriOrLabel;
impl PartialEq<&str> for RoofLoadPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RoofLoadPropertyIri || *other == ROOF_LOAD_PROPERTY_LABEL
	}
}
impl PartialEq<RoofLoadPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RoofLoadPropertyIriOrLabel) -> bool {
		*self == RoofLoadPropertyIri || *self == ROOF_LOAD_PROPERTY_LABEL
	}
}
