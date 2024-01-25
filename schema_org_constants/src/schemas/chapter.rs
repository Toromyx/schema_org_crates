/// <https://schema.org/Chapter>
pub const CHAPTER_IRI_HTTP: &str = "http://schema.org/Chapter";
/// <https://schema.org/Chapter>
pub const CHAPTER_IRI_HTTPS: &str = "https://schema.org/Chapter";
/// <https://schema.org/Chapter>
pub const CHAPTER_LABEL: &str = "Chapter";
pub struct ChapterIri;
impl PartialEq<&str> for ChapterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHAPTER_IRI_HTTP || *other == CHAPTER_IRI_HTTPS
	}
}
impl PartialEq<ChapterIri> for &str {
	fn eq(&self, other: &ChapterIri) -> bool {
		*self == CHAPTER_IRI_HTTP || *self == CHAPTER_IRI_HTTPS
	}
}
pub struct ChapterIriOrLabel;
impl PartialEq<&str> for ChapterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChapterIri || *other == CHAPTER_LABEL
	}
}
impl PartialEq<ChapterIriOrLabel> for &str {
	fn eq(&self, other: &ChapterIriOrLabel) -> bool {
		*self == ChapterIri || *self == CHAPTER_LABEL
	}
}
