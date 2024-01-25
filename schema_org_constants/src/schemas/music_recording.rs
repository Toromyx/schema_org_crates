/// <https://schema.org/MusicRecording>
pub const MUSIC_RECORDING_IRI_HTTP: &str = "http://schema.org/MusicRecording";
/// <https://schema.org/MusicRecording>
pub const MUSIC_RECORDING_IRI_HTTPS: &str = "https://schema.org/MusicRecording";
/// <https://schema.org/MusicRecording>
pub const MUSIC_RECORDING_LABEL: &str = "MusicRecording";
pub struct MusicRecordingIri;
impl PartialEq<&str> for MusicRecordingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_RECORDING_IRI_HTTP || *other == MUSIC_RECORDING_IRI_HTTPS
	}
}
impl PartialEq<MusicRecordingIri> for &str {
	fn eq(&self, other: &MusicRecordingIri) -> bool {
		*self == MUSIC_RECORDING_IRI_HTTP || *self == MUSIC_RECORDING_IRI_HTTPS
	}
}
pub struct MusicRecordingIriOrLabel;
impl PartialEq<&str> for MusicRecordingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicRecordingIri || *other == MUSIC_RECORDING_LABEL
	}
}
impl PartialEq<MusicRecordingIriOrLabel> for &str {
	fn eq(&self, other: &MusicRecordingIriOrLabel) -> bool {
		*self == MusicRecordingIri || *self == MUSIC_RECORDING_LABEL
	}
}
