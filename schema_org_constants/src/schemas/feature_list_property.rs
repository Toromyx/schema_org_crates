/// <https://schema.org/featureList>
pub const FEATURE_LIST_PROPERTY_IRI_HTTP: &str = "http://schema.org/featureList";
/// <https://schema.org/featureList>
pub const FEATURE_LIST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/featureList";
/// <https://schema.org/featureList>
pub const FEATURE_LIST_PROPERTY_LABEL: &str = "featureList";
pub struct FeatureListPropertyIri;
impl PartialEq<&str> for FeatureListPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FEATURE_LIST_PROPERTY_IRI_HTTP || *other == FEATURE_LIST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FeatureListPropertyIri> for &str {
	fn eq(&self, other: &FeatureListPropertyIri) -> bool {
		*self == FEATURE_LIST_PROPERTY_IRI_HTTP || *self == FEATURE_LIST_PROPERTY_IRI_HTTPS
	}
}
pub struct FeatureListPropertyIriOrLabel;
impl PartialEq<&str> for FeatureListPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FeatureListPropertyIri || *other == FEATURE_LIST_PROPERTY_LABEL
	}
}
impl PartialEq<FeatureListPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FeatureListPropertyIriOrLabel) -> bool {
		*self == FeatureListPropertyIri || *self == FEATURE_LIST_PROPERTY_LABEL
	}
}
