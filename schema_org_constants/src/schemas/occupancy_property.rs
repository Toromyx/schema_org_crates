/// <https://schema.org/occupancy>
pub const OCCUPANCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/occupancy";
/// <https://schema.org/occupancy>
pub const OCCUPANCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/occupancy";
/// <https://schema.org/occupancy>
pub const OCCUPANCY_PROPERTY_LABEL: &str = "occupancy";
pub struct OccupancyPropertyIri;
impl PartialEq<&str> for OccupancyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OCCUPANCY_PROPERTY_IRI_HTTP || *other == OCCUPANCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OccupancyPropertyIri> for &str {
	fn eq(&self, other: &OccupancyPropertyIri) -> bool {
		*self == OCCUPANCY_PROPERTY_IRI_HTTP || *self == OCCUPANCY_PROPERTY_IRI_HTTPS
	}
}
pub struct OccupancyPropertyIriOrLabel;
impl PartialEq<&str> for OccupancyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OccupancyPropertyIri || *other == OCCUPANCY_PROPERTY_LABEL
	}
}
impl PartialEq<OccupancyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OccupancyPropertyIriOrLabel) -> bool {
		*self == OccupancyPropertyIri || *self == OCCUPANCY_PROPERTY_LABEL
	}
}
