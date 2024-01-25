/// <https://schema.org/floorLevel>
pub const FLOOR_LEVEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/floorLevel";
/// <https://schema.org/floorLevel>
pub const FLOOR_LEVEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/floorLevel";
/// <https://schema.org/floorLevel>
pub const FLOOR_LEVEL_PROPERTY_LABEL: &str = "floorLevel";
pub struct FloorLevelPropertyIri;
impl PartialEq<&str> for FloorLevelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLOOR_LEVEL_PROPERTY_IRI_HTTP || *other == FLOOR_LEVEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FloorLevelPropertyIri> for &str {
	fn eq(&self, other: &FloorLevelPropertyIri) -> bool {
		*self == FLOOR_LEVEL_PROPERTY_IRI_HTTP || *self == FLOOR_LEVEL_PROPERTY_IRI_HTTPS
	}
}
pub struct FloorLevelPropertyIriOrLabel;
impl PartialEq<&str> for FloorLevelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FloorLevelPropertyIri || *other == FLOOR_LEVEL_PROPERTY_LABEL
	}
}
impl PartialEq<FloorLevelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FloorLevelPropertyIriOrLabel) -> bool {
		*self == FloorLevelPropertyIri || *self == FLOOR_LEVEL_PROPERTY_LABEL
	}
}
