/// <https://schema.org/MusicRelease>
pub const MUSIC_RELEASE_IRI_HTTP: &str = "http://schema.org/MusicRelease";
/// <https://schema.org/MusicRelease>
pub const MUSIC_RELEASE_IRI_HTTPS: &str = "https://schema.org/MusicRelease";
/// <https://schema.org/MusicRelease>
pub const MUSIC_RELEASE_LABEL: &str = "MusicRelease";
pub struct MusicReleaseIri;
impl PartialEq<&str> for MusicReleaseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_RELEASE_IRI_HTTP || *other == MUSIC_RELEASE_IRI_HTTPS
	}
}
impl PartialEq<MusicReleaseIri> for &str {
	fn eq(&self, other: &MusicReleaseIri) -> bool {
		*self == MUSIC_RELEASE_IRI_HTTP || *self == MUSIC_RELEASE_IRI_HTTPS
	}
}
pub struct MusicReleaseIriOrLabel;
impl PartialEq<&str> for MusicReleaseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicReleaseIri || *other == MUSIC_RELEASE_LABEL
	}
}
impl PartialEq<MusicReleaseIriOrLabel> for &str {
	fn eq(&self, other: &MusicReleaseIriOrLabel) -> bool {
		*self == MusicReleaseIri || *self == MUSIC_RELEASE_LABEL
	}
}
