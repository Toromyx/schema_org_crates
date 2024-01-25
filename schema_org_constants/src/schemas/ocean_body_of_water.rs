/// <https://schema.org/OceanBodyOfWater>
pub const OCEAN_BODY_OF_WATER_IRI_HTTP: &str = "http://schema.org/OceanBodyOfWater";
/// <https://schema.org/OceanBodyOfWater>
pub const OCEAN_BODY_OF_WATER_IRI_HTTPS: &str = "https://schema.org/OceanBodyOfWater";
/// <https://schema.org/OceanBodyOfWater>
pub const OCEAN_BODY_OF_WATER_LABEL: &str = "OceanBodyOfWater";
pub struct OceanBodyOfWaterIri;
impl PartialEq<&str> for OceanBodyOfWaterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OCEAN_BODY_OF_WATER_IRI_HTTP || *other == OCEAN_BODY_OF_WATER_IRI_HTTPS
	}
}
impl PartialEq<OceanBodyOfWaterIri> for &str {
	fn eq(&self, other: &OceanBodyOfWaterIri) -> bool {
		*self == OCEAN_BODY_OF_WATER_IRI_HTTP || *self == OCEAN_BODY_OF_WATER_IRI_HTTPS
	}
}
pub struct OceanBodyOfWaterIriOrLabel;
impl PartialEq<&str> for OceanBodyOfWaterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OceanBodyOfWaterIri || *other == OCEAN_BODY_OF_WATER_LABEL
	}
}
impl PartialEq<OceanBodyOfWaterIriOrLabel> for &str {
	fn eq(&self, other: &OceanBodyOfWaterIriOrLabel) -> bool {
		*self == OceanBodyOfWaterIri || *self == OCEAN_BODY_OF_WATER_LABEL
	}
}
