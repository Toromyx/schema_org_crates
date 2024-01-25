/// <https://schema.org/SeaBodyOfWater>
pub const SEA_BODY_OF_WATER_IRI_HTTP: &str = "http://schema.org/SeaBodyOfWater";
/// <https://schema.org/SeaBodyOfWater>
pub const SEA_BODY_OF_WATER_IRI_HTTPS: &str = "https://schema.org/SeaBodyOfWater";
/// <https://schema.org/SeaBodyOfWater>
pub const SEA_BODY_OF_WATER_LABEL: &str = "SeaBodyOfWater";
pub struct SeaBodyOfWaterIri;
impl PartialEq<&str> for SeaBodyOfWaterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEA_BODY_OF_WATER_IRI_HTTP || *other == SEA_BODY_OF_WATER_IRI_HTTPS
	}
}
impl PartialEq<SeaBodyOfWaterIri> for &str {
	fn eq(&self, other: &SeaBodyOfWaterIri) -> bool {
		*self == SEA_BODY_OF_WATER_IRI_HTTP || *self == SEA_BODY_OF_WATER_IRI_HTTPS
	}
}
pub struct SeaBodyOfWaterIriOrLabel;
impl PartialEq<&str> for SeaBodyOfWaterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeaBodyOfWaterIri || *other == SEA_BODY_OF_WATER_LABEL
	}
}
impl PartialEq<SeaBodyOfWaterIriOrLabel> for &str {
	fn eq(&self, other: &SeaBodyOfWaterIriOrLabel) -> bool {
		*self == SeaBodyOfWaterIri || *self == SEA_BODY_OF_WATER_LABEL
	}
}
