/// <https://schema.org/AmpStory>
pub const AMP_STORY_IRI_HTTP: &str = "http://schema.org/AmpStory";
/// <https://schema.org/AmpStory>
pub const AMP_STORY_IRI_HTTPS: &str = "https://schema.org/AmpStory";
/// <https://schema.org/AmpStory>
pub const AMP_STORY_LABEL: &str = "AmpStory";
pub struct AmpStoryIri;
impl PartialEq<&str> for AmpStoryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AMP_STORY_IRI_HTTP || *other == AMP_STORY_IRI_HTTPS
	}
}
impl PartialEq<AmpStoryIri> for &str {
	fn eq(&self, other: &AmpStoryIri) -> bool {
		*self == AMP_STORY_IRI_HTTP || *self == AMP_STORY_IRI_HTTPS
	}
}
pub struct AmpStoryIriOrLabel;
impl PartialEq<&str> for AmpStoryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AmpStoryIri || *other == AMP_STORY_LABEL
	}
}
impl PartialEq<AmpStoryIriOrLabel> for &str {
	fn eq(&self, other: &AmpStoryIriOrLabel) -> bool {
		*self == AmpStoryIri || *self == AMP_STORY_LABEL
	}
}
