/// <https://schema.org/RiverBodyOfWater>
pub const RIVER_BODY_OF_WATER_IRI_HTTP: &str = "http://schema.org/RiverBodyOfWater";
/// <https://schema.org/RiverBodyOfWater>
pub const RIVER_BODY_OF_WATER_IRI_HTTPS: &str = "https://schema.org/RiverBodyOfWater";
/// <https://schema.org/RiverBodyOfWater>
pub const RIVER_BODY_OF_WATER_LABEL: &str = "RiverBodyOfWater";
pub struct RiverBodyOfWaterIri;
impl PartialEq<&str> for RiverBodyOfWaterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RIVER_BODY_OF_WATER_IRI_HTTP || *other == RIVER_BODY_OF_WATER_IRI_HTTPS
	}
}
impl PartialEq<RiverBodyOfWaterIri> for &str {
	fn eq(&self, other: &RiverBodyOfWaterIri) -> bool {
		*self == RIVER_BODY_OF_WATER_IRI_HTTP || *self == RIVER_BODY_OF_WATER_IRI_HTTPS
	}
}
pub struct RiverBodyOfWaterIriOrLabel;
impl PartialEq<&str> for RiverBodyOfWaterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RiverBodyOfWaterIri || *other == RIVER_BODY_OF_WATER_LABEL
	}
}
impl PartialEq<RiverBodyOfWaterIriOrLabel> for &str {
	fn eq(&self, other: &RiverBodyOfWaterIriOrLabel) -> bool {
		*self == RiverBodyOfWaterIri || *self == RIVER_BODY_OF_WATER_LABEL
	}
}
