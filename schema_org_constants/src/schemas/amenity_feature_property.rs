/// <https://schema.org/amenityFeature>
pub const AMENITY_FEATURE_PROPERTY_IRI_HTTP: &str = "http://schema.org/amenityFeature";
/// <https://schema.org/amenityFeature>
pub const AMENITY_FEATURE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/amenityFeature";
/// <https://schema.org/amenityFeature>
pub const AMENITY_FEATURE_PROPERTY_LABEL: &str = "amenityFeature";
pub struct AmenityFeaturePropertyIri;
impl PartialEq<&str> for AmenityFeaturePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AMENITY_FEATURE_PROPERTY_IRI_HTTP || *other == AMENITY_FEATURE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AmenityFeaturePropertyIri> for &str {
	fn eq(&self, other: &AmenityFeaturePropertyIri) -> bool {
		*self == AMENITY_FEATURE_PROPERTY_IRI_HTTP || *self == AMENITY_FEATURE_PROPERTY_IRI_HTTPS
	}
}
pub struct AmenityFeaturePropertyIriOrLabel;
impl PartialEq<&str> for AmenityFeaturePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AmenityFeaturePropertyIri || *other == AMENITY_FEATURE_PROPERTY_LABEL
	}
}
impl PartialEq<AmenityFeaturePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AmenityFeaturePropertyIriOrLabel) -> bool {
		*self == AmenityFeaturePropertyIri || *self == AMENITY_FEATURE_PROPERTY_LABEL
	}
}
