/// <https://schema.org/MusicReleaseFormatType>
pub const MUSIC_RELEASE_FORMAT_TYPE_IRI_HTTP: &str = "http://schema.org/MusicReleaseFormatType";
/// <https://schema.org/MusicReleaseFormatType>
pub const MUSIC_RELEASE_FORMAT_TYPE_IRI_HTTPS: &str = "https://schema.org/MusicReleaseFormatType";
/// <https://schema.org/MusicReleaseFormatType>
pub const MUSIC_RELEASE_FORMAT_TYPE_LABEL: &str = "MusicReleaseFormatType";
pub struct MusicReleaseFormatTypeIri;
impl PartialEq<&str> for MusicReleaseFormatTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_RELEASE_FORMAT_TYPE_IRI_HTTP
			|| *other == MUSIC_RELEASE_FORMAT_TYPE_IRI_HTTPS
	}
}
impl PartialEq<MusicReleaseFormatTypeIri> for &str {
	fn eq(&self, other: &MusicReleaseFormatTypeIri) -> bool {
		*self == MUSIC_RELEASE_FORMAT_TYPE_IRI_HTTP || *self == MUSIC_RELEASE_FORMAT_TYPE_IRI_HTTPS
	}
}
pub struct MusicReleaseFormatTypeIriOrLabel;
impl PartialEq<&str> for MusicReleaseFormatTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicReleaseFormatTypeIri || *other == MUSIC_RELEASE_FORMAT_TYPE_LABEL
	}
}
impl PartialEq<MusicReleaseFormatTypeIriOrLabel> for &str {
	fn eq(&self, other: &MusicReleaseFormatTypeIriOrLabel) -> bool {
		*self == MusicReleaseFormatTypeIri || *self == MUSIC_RELEASE_FORMAT_TYPE_LABEL
	}
}
