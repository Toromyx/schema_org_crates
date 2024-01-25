/// <https://schema.org/dietFeatures>
pub const DIET_FEATURES_PROPERTY_IRI_HTTP: &str = "http://schema.org/dietFeatures";
/// <https://schema.org/dietFeatures>
pub const DIET_FEATURES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dietFeatures";
/// <https://schema.org/dietFeatures>
pub const DIET_FEATURES_PROPERTY_LABEL: &str = "dietFeatures";
pub struct DietFeaturesPropertyIri;
impl PartialEq<&str> for DietFeaturesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIET_FEATURES_PROPERTY_IRI_HTTP || *other == DIET_FEATURES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DietFeaturesPropertyIri> for &str {
	fn eq(&self, other: &DietFeaturesPropertyIri) -> bool {
		*self == DIET_FEATURES_PROPERTY_IRI_HTTP || *self == DIET_FEATURES_PROPERTY_IRI_HTTPS
	}
}
pub struct DietFeaturesPropertyIriOrLabel;
impl PartialEq<&str> for DietFeaturesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DietFeaturesPropertyIri || *other == DIET_FEATURES_PROPERTY_LABEL
	}
}
impl PartialEq<DietFeaturesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DietFeaturesPropertyIriOrLabel) -> bool {
		*self == DietFeaturesPropertyIri || *self == DIET_FEATURES_PROPERTY_LABEL
	}
}
