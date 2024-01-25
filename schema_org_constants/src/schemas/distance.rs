/// <https://schema.org/Distance>
pub const DISTANCE_IRI_HTTP: &str = "http://schema.org/Distance";
/// <https://schema.org/Distance>
pub const DISTANCE_IRI_HTTPS: &str = "https://schema.org/Distance";
/// <https://schema.org/Distance>
pub const DISTANCE_LABEL: &str = "Distance";
pub struct DistanceIri;
impl PartialEq<&str> for DistanceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISTANCE_IRI_HTTP || *other == DISTANCE_IRI_HTTPS
	}
}
impl PartialEq<DistanceIri> for &str {
	fn eq(&self, other: &DistanceIri) -> bool {
		*self == DISTANCE_IRI_HTTP || *self == DISTANCE_IRI_HTTPS
	}
}
pub struct DistanceIriOrLabel;
impl PartialEq<&str> for DistanceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DistanceIri || *other == DISTANCE_LABEL
	}
}
impl PartialEq<DistanceIriOrLabel> for &str {
	fn eq(&self, other: &DistanceIriOrLabel) -> bool {
		*self == DistanceIri || *self == DISTANCE_LABEL
	}
}
