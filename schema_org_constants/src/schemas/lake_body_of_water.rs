/// <https://schema.org/LakeBodyOfWater>
pub const LAKE_BODY_OF_WATER_IRI_HTTP: &str = "http://schema.org/LakeBodyOfWater";
/// <https://schema.org/LakeBodyOfWater>
pub const LAKE_BODY_OF_WATER_IRI_HTTPS: &str = "https://schema.org/LakeBodyOfWater";
/// <https://schema.org/LakeBodyOfWater>
pub const LAKE_BODY_OF_WATER_LABEL: &str = "LakeBodyOfWater";
pub struct LakeBodyOfWaterIri;
impl PartialEq<&str> for LakeBodyOfWaterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LAKE_BODY_OF_WATER_IRI_HTTP || *other == LAKE_BODY_OF_WATER_IRI_HTTPS
	}
}
impl PartialEq<LakeBodyOfWaterIri> for &str {
	fn eq(&self, other: &LakeBodyOfWaterIri) -> bool {
		*self == LAKE_BODY_OF_WATER_IRI_HTTP || *self == LAKE_BODY_OF_WATER_IRI_HTTPS
	}
}
pub struct LakeBodyOfWaterIriOrLabel;
impl PartialEq<&str> for LakeBodyOfWaterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LakeBodyOfWaterIri || *other == LAKE_BODY_OF_WATER_LABEL
	}
}
impl PartialEq<LakeBodyOfWaterIriOrLabel> for &str {
	fn eq(&self, other: &LakeBodyOfWaterIriOrLabel) -> bool {
		*self == LakeBodyOfWaterIri || *self == LAKE_BODY_OF_WATER_LABEL
	}
}
