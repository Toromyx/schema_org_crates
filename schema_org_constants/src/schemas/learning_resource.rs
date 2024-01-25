/// <https://schema.org/LearningResource>
pub const LEARNING_RESOURCE_IRI_HTTP: &str = "http://schema.org/LearningResource";
/// <https://schema.org/LearningResource>
pub const LEARNING_RESOURCE_IRI_HTTPS: &str = "https://schema.org/LearningResource";
/// <https://schema.org/LearningResource>
pub const LEARNING_RESOURCE_LABEL: &str = "LearningResource";
pub struct LearningResourceIri;
impl PartialEq<&str> for LearningResourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEARNING_RESOURCE_IRI_HTTP || *other == LEARNING_RESOURCE_IRI_HTTPS
	}
}
impl PartialEq<LearningResourceIri> for &str {
	fn eq(&self, other: &LearningResourceIri) -> bool {
		*self == LEARNING_RESOURCE_IRI_HTTP || *self == LEARNING_RESOURCE_IRI_HTTPS
	}
}
pub struct LearningResourceIriOrLabel;
impl PartialEq<&str> for LearningResourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LearningResourceIri || *other == LEARNING_RESOURCE_LABEL
	}
}
impl PartialEq<LearningResourceIriOrLabel> for &str {
	fn eq(&self, other: &LearningResourceIriOrLabel) -> bool {
		*self == LearningResourceIri || *self == LEARNING_RESOURCE_LABEL
	}
}
