/// <https://schema.org/ShortStory>
pub const SHORT_STORY_IRI_HTTP: &str = "http://schema.org/ShortStory";
/// <https://schema.org/ShortStory>
pub const SHORT_STORY_IRI_HTTPS: &str = "https://schema.org/ShortStory";
/// <https://schema.org/ShortStory>
pub const SHORT_STORY_LABEL: &str = "ShortStory";
pub struct ShortStoryIri;
impl PartialEq<&str> for ShortStoryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHORT_STORY_IRI_HTTP || *other == SHORT_STORY_IRI_HTTPS
	}
}
impl PartialEq<ShortStoryIri> for &str {
	fn eq(&self, other: &ShortStoryIri) -> bool {
		*self == SHORT_STORY_IRI_HTTP || *self == SHORT_STORY_IRI_HTTPS
	}
}
pub struct ShortStoryIriOrLabel;
impl PartialEq<&str> for ShortStoryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShortStoryIri || *other == SHORT_STORY_LABEL
	}
}
impl PartialEq<ShortStoryIriOrLabel> for &str {
	fn eq(&self, other: &ShortStoryIriOrLabel) -> bool {
		*self == ShortStoryIri || *self == SHORT_STORY_LABEL
	}
}
