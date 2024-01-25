/// <https://schema.org/floorLimit>
pub const FLOOR_LIMIT_PROPERTY_IRI_HTTP: &str = "http://schema.org/floorLimit";
/// <https://schema.org/floorLimit>
pub const FLOOR_LIMIT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/floorLimit";
/// <https://schema.org/floorLimit>
pub const FLOOR_LIMIT_PROPERTY_LABEL: &str = "floorLimit";
pub struct FloorLimitPropertyIri;
impl PartialEq<&str> for FloorLimitPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLOOR_LIMIT_PROPERTY_IRI_HTTP || *other == FLOOR_LIMIT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FloorLimitPropertyIri> for &str {
	fn eq(&self, other: &FloorLimitPropertyIri) -> bool {
		*self == FLOOR_LIMIT_PROPERTY_IRI_HTTP || *self == FLOOR_LIMIT_PROPERTY_IRI_HTTPS
	}
}
pub struct FloorLimitPropertyIriOrLabel;
impl PartialEq<&str> for FloorLimitPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FloorLimitPropertyIri || *other == FLOOR_LIMIT_PROPERTY_LABEL
	}
}
impl PartialEq<FloorLimitPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FloorLimitPropertyIriOrLabel) -> bool {
		*self == FloorLimitPropertyIri || *self == FLOOR_LIMIT_PROPERTY_LABEL
	}
}
