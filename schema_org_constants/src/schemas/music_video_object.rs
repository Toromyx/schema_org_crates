/// <https://schema.org/MusicVideoObject>
pub const MUSIC_VIDEO_OBJECT_IRI_HTTP: &str = "http://schema.org/MusicVideoObject";
/// <https://schema.org/MusicVideoObject>
pub const MUSIC_VIDEO_OBJECT_IRI_HTTPS: &str = "https://schema.org/MusicVideoObject";
/// <https://schema.org/MusicVideoObject>
pub const MUSIC_VIDEO_OBJECT_LABEL: &str = "MusicVideoObject";
pub struct MusicVideoObjectIri;
impl PartialEq<&str> for MusicVideoObjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_VIDEO_OBJECT_IRI_HTTP || *other == MUSIC_VIDEO_OBJECT_IRI_HTTPS
	}
}
impl PartialEq<MusicVideoObjectIri> for &str {
	fn eq(&self, other: &MusicVideoObjectIri) -> bool {
		*self == MUSIC_VIDEO_OBJECT_IRI_HTTP || *self == MUSIC_VIDEO_OBJECT_IRI_HTTPS
	}
}
pub struct MusicVideoObjectIriOrLabel;
impl PartialEq<&str> for MusicVideoObjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicVideoObjectIri || *other == MUSIC_VIDEO_OBJECT_LABEL
	}
}
impl PartialEq<MusicVideoObjectIriOrLabel> for &str {
	fn eq(&self, other: &MusicVideoObjectIriOrLabel) -> bool {
		*self == MusicVideoObjectIri || *self == MUSIC_VIDEO_OBJECT_LABEL
	}
}
