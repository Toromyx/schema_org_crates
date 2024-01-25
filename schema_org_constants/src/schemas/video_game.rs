/// <https://schema.org/VideoGame>
pub const VIDEO_GAME_IRI_HTTP: &str = "http://schema.org/VideoGame";
/// <https://schema.org/VideoGame>
pub const VIDEO_GAME_IRI_HTTPS: &str = "https://schema.org/VideoGame";
/// <https://schema.org/VideoGame>
pub const VIDEO_GAME_LABEL: &str = "VideoGame";
pub struct VideoGameIri;
impl PartialEq<&str> for VideoGameIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_GAME_IRI_HTTP || *other == VIDEO_GAME_IRI_HTTPS
	}
}
impl PartialEq<VideoGameIri> for &str {
	fn eq(&self, other: &VideoGameIri) -> bool {
		*self == VIDEO_GAME_IRI_HTTP || *self == VIDEO_GAME_IRI_HTTPS
	}
}
pub struct VideoGameIriOrLabel;
impl PartialEq<&str> for VideoGameIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoGameIri || *other == VIDEO_GAME_LABEL
	}
}
impl PartialEq<VideoGameIriOrLabel> for &str {
	fn eq(&self, other: &VideoGameIriOrLabel) -> bool {
		*self == VideoGameIri || *self == VIDEO_GAME_LABEL
	}
}
