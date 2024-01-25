/// <https://schema.org/LocationFeatureSpecification>
pub const LOCATION_FEATURE_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/LocationFeatureSpecification";
/// <https://schema.org/LocationFeatureSpecification>
pub const LOCATION_FEATURE_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/LocationFeatureSpecification";
/// <https://schema.org/LocationFeatureSpecification>
pub const LOCATION_FEATURE_SPECIFICATION_LABEL: &str = "LocationFeatureSpecification";
pub struct LocationFeatureSpecificationIri;
impl PartialEq<&str> for LocationFeatureSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOCATION_FEATURE_SPECIFICATION_IRI_HTTP
			|| *other == LOCATION_FEATURE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<LocationFeatureSpecificationIri> for &str {
	fn eq(&self, other: &LocationFeatureSpecificationIri) -> bool {
		*self == LOCATION_FEATURE_SPECIFICATION_IRI_HTTP
			|| *self == LOCATION_FEATURE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct LocationFeatureSpecificationIriOrLabel;
impl PartialEq<&str> for LocationFeatureSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LocationFeatureSpecificationIri || *other == LOCATION_FEATURE_SPECIFICATION_LABEL
	}
}
impl PartialEq<LocationFeatureSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &LocationFeatureSpecificationIriOrLabel) -> bool {
		*self == LocationFeatureSpecificationIri || *self == LOCATION_FEATURE_SPECIFICATION_LABEL
	}
}
