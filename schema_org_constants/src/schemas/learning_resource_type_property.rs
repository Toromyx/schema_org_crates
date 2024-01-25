/// <https://schema.org/learningResourceType>
pub const LEARNING_RESOURCE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/learningResourceType";
/// <https://schema.org/learningResourceType>
pub const LEARNING_RESOURCE_TYPE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/learningResourceType";
/// <https://schema.org/learningResourceType>
pub const LEARNING_RESOURCE_TYPE_PROPERTY_LABEL: &str = "learningResourceType";
pub struct LearningResourceTypePropertyIri;
impl PartialEq<&str> for LearningResourceTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEARNING_RESOURCE_TYPE_PROPERTY_IRI_HTTP
			|| *other == LEARNING_RESOURCE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LearningResourceTypePropertyIri> for &str {
	fn eq(&self, other: &LearningResourceTypePropertyIri) -> bool {
		*self == LEARNING_RESOURCE_TYPE_PROPERTY_IRI_HTTP
			|| *self == LEARNING_RESOURCE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct LearningResourceTypePropertyIriOrLabel;
impl PartialEq<&str> for LearningResourceTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LearningResourceTypePropertyIri || *other == LEARNING_RESOURCE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<LearningResourceTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LearningResourceTypePropertyIriOrLabel) -> bool {
		*self == LearningResourceTypePropertyIri || *self == LEARNING_RESOURCE_TYPE_PROPERTY_LABEL
	}
}
