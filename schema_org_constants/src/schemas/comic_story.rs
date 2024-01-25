/// <https://schema.org/ComicStory>
pub const COMIC_STORY_IRI_HTTP: &str = "http://schema.org/ComicStory";
/// <https://schema.org/ComicStory>
pub const COMIC_STORY_IRI_HTTPS: &str = "https://schema.org/ComicStory";
/// <https://schema.org/ComicStory>
pub const COMIC_STORY_LABEL: &str = "ComicStory";
pub struct ComicStoryIri;
impl PartialEq<&str> for ComicStoryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMIC_STORY_IRI_HTTP || *other == COMIC_STORY_IRI_HTTPS
	}
}
impl PartialEq<ComicStoryIri> for &str {
	fn eq(&self, other: &ComicStoryIri) -> bool {
		*self == COMIC_STORY_IRI_HTTP || *self == COMIC_STORY_IRI_HTTPS
	}
}
pub struct ComicStoryIriOrLabel;
impl PartialEq<&str> for ComicStoryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComicStoryIri || *other == COMIC_STORY_LABEL
	}
}
impl PartialEq<ComicStoryIriOrLabel> for &str {
	fn eq(&self, other: &ComicStoryIriOrLabel) -> bool {
		*self == ComicStoryIri || *self == COMIC_STORY_LABEL
	}
}
