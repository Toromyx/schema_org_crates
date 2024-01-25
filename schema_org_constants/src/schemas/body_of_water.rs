/// <https://schema.org/BodyOfWater>
pub const BODY_OF_WATER_IRI_HTTP: &str = "http://schema.org/BodyOfWater";
/// <https://schema.org/BodyOfWater>
pub const BODY_OF_WATER_IRI_HTTPS: &str = "https://schema.org/BodyOfWater";
/// <https://schema.org/BodyOfWater>
pub const BODY_OF_WATER_LABEL: &str = "BodyOfWater";
pub struct BodyOfWaterIri;
impl PartialEq<&str> for BodyOfWaterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_OF_WATER_IRI_HTTP || *other == BODY_OF_WATER_IRI_HTTPS
	}
}
impl PartialEq<BodyOfWaterIri> for &str {
	fn eq(&self, other: &BodyOfWaterIri) -> bool {
		*self == BODY_OF_WATER_IRI_HTTP || *self == BODY_OF_WATER_IRI_HTTPS
	}
}
pub struct BodyOfWaterIriOrLabel;
impl PartialEq<&str> for BodyOfWaterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyOfWaterIri || *other == BODY_OF_WATER_LABEL
	}
}
impl PartialEq<BodyOfWaterIriOrLabel> for &str {
	fn eq(&self, other: &BodyOfWaterIriOrLabel) -> bool {
		*self == BodyOfWaterIri || *self == BODY_OF_WATER_LABEL
	}
}
