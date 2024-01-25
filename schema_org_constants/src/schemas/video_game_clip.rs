/// <https://schema.org/VideoGameClip>
pub const VIDEO_GAME_CLIP_IRI_HTTP: &str = "http://schema.org/VideoGameClip";
/// <https://schema.org/VideoGameClip>
pub const VIDEO_GAME_CLIP_IRI_HTTPS: &str = "https://schema.org/VideoGameClip";
/// <https://schema.org/VideoGameClip>
pub const VIDEO_GAME_CLIP_LABEL: &str = "VideoGameClip";
pub struct VideoGameClipIri;
impl PartialEq<&str> for VideoGameClipIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_GAME_CLIP_IRI_HTTP || *other == VIDEO_GAME_CLIP_IRI_HTTPS
	}
}
impl PartialEq<VideoGameClipIri> for &str {
	fn eq(&self, other: &VideoGameClipIri) -> bool {
		*self == VIDEO_GAME_CLIP_IRI_HTTP || *self == VIDEO_GAME_CLIP_IRI_HTTPS
	}
}
pub struct VideoGameClipIriOrLabel;
impl PartialEq<&str> for VideoGameClipIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoGameClipIri || *other == VIDEO_GAME_CLIP_LABEL
	}
}
impl PartialEq<VideoGameClipIriOrLabel> for &str {
	fn eq(&self, other: &VideoGameClipIriOrLabel) -> bool {
		*self == VideoGameClipIri || *self == VIDEO_GAME_CLIP_LABEL
	}
}
