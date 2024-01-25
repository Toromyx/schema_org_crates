/// <https://schema.org/floorSize>
pub const FLOOR_SIZE_PROPERTY_IRI_HTTP: &str = "http://schema.org/floorSize";
/// <https://schema.org/floorSize>
pub const FLOOR_SIZE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/floorSize";
/// <https://schema.org/floorSize>
pub const FLOOR_SIZE_PROPERTY_LABEL: &str = "floorSize";
pub struct FloorSizePropertyIri;
impl PartialEq<&str> for FloorSizePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLOOR_SIZE_PROPERTY_IRI_HTTP || *other == FLOOR_SIZE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FloorSizePropertyIri> for &str {
	fn eq(&self, other: &FloorSizePropertyIri) -> bool {
		*self == FLOOR_SIZE_PROPERTY_IRI_HTTP || *self == FLOOR_SIZE_PROPERTY_IRI_HTTPS
	}
}
pub struct FloorSizePropertyIriOrLabel;
impl PartialEq<&str> for FloorSizePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FloorSizePropertyIri || *other == FLOOR_SIZE_PROPERTY_LABEL
	}
}
impl PartialEq<FloorSizePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FloorSizePropertyIriOrLabel) -> bool {
		*self == FloorSizePropertyIri || *self == FLOOR_SIZE_PROPERTY_LABEL
	}
}
