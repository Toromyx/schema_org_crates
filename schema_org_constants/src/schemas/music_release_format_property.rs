/// <https://schema.org/musicReleaseFormat>
pub const MUSIC_RELEASE_FORMAT_PROPERTY_IRI_HTTP: &str = "http://schema.org/musicReleaseFormat";
/// <https://schema.org/musicReleaseFormat>
pub const MUSIC_RELEASE_FORMAT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/musicReleaseFormat";
/// <https://schema.org/musicReleaseFormat>
pub const MUSIC_RELEASE_FORMAT_PROPERTY_LABEL: &str = "musicReleaseFormat";
pub struct MusicReleaseFormatPropertyIri;
impl PartialEq<&str> for MusicReleaseFormatPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_RELEASE_FORMAT_PROPERTY_IRI_HTTP
			|| *other == MUSIC_RELEASE_FORMAT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MusicReleaseFormatPropertyIri> for &str {
	fn eq(&self, other: &MusicReleaseFormatPropertyIri) -> bool {
		*self == MUSIC_RELEASE_FORMAT_PROPERTY_IRI_HTTP
			|| *self == MUSIC_RELEASE_FORMAT_PROPERTY_IRI_HTTPS
	}
}
pub struct MusicReleaseFormatPropertyIriOrLabel;
impl PartialEq<&str> for MusicReleaseFormatPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicReleaseFormatPropertyIri || *other == MUSIC_RELEASE_FORMAT_PROPERTY_LABEL
	}
}
impl PartialEq<MusicReleaseFormatPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MusicReleaseFormatPropertyIriOrLabel) -> bool {
		*self == MusicReleaseFormatPropertyIri || *self == MUSIC_RELEASE_FORMAT_PROPERTY_LABEL
	}
}
