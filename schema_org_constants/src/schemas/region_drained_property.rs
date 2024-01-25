/// <https://schema.org/regionDrained>
pub const REGION_DRAINED_PROPERTY_IRI_HTTP: &str = "http://schema.org/regionDrained";
/// <https://schema.org/regionDrained>
pub const REGION_DRAINED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/regionDrained";
/// <https://schema.org/regionDrained>
pub const REGION_DRAINED_PROPERTY_LABEL: &str = "regionDrained";
pub struct RegionDrainedPropertyIri;
impl PartialEq<&str> for RegionDrainedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REGION_DRAINED_PROPERTY_IRI_HTTP || *other == REGION_DRAINED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RegionDrainedPropertyIri> for &str {
	fn eq(&self, other: &RegionDrainedPropertyIri) -> bool {
		*self == REGION_DRAINED_PROPERTY_IRI_HTTP || *self == REGION_DRAINED_PROPERTY_IRI_HTTPS
	}
}
pub struct RegionDrainedPropertyIriOrLabel;
impl PartialEq<&str> for RegionDrainedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RegionDrainedPropertyIri || *other == REGION_DRAINED_PROPERTY_LABEL
	}
}
impl PartialEq<RegionDrainedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RegionDrainedPropertyIriOrLabel) -> bool {
		*self == RegionDrainedPropertyIri || *self == REGION_DRAINED_PROPERTY_LABEL
	}
}
