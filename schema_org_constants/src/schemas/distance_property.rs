/// <https://schema.org/distance>
pub const DISTANCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/distance";
/// <https://schema.org/distance>
pub const DISTANCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/distance";
/// <https://schema.org/distance>
pub const DISTANCE_PROPERTY_LABEL: &str = "distance";
pub struct DistancePropertyIri;
impl PartialEq<&str> for DistancePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISTANCE_PROPERTY_IRI_HTTP || *other == DISTANCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DistancePropertyIri> for &str {
	fn eq(&self, other: &DistancePropertyIri) -> bool {
		*self == DISTANCE_PROPERTY_IRI_HTTP || *self == DISTANCE_PROPERTY_IRI_HTTPS
	}
}
pub struct DistancePropertyIriOrLabel;
impl PartialEq<&str> for DistancePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DistancePropertyIri || *other == DISTANCE_PROPERTY_LABEL
	}
}
impl PartialEq<DistancePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DistancePropertyIriOrLabel) -> bool {
		*self == DistancePropertyIri || *self == DISTANCE_PROPERTY_LABEL
	}
}
