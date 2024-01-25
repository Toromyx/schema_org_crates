/// <https://schema.org/AudioObjectSnapshot>
pub const AUDIO_OBJECT_SNAPSHOT_IRI_HTTP: &str = "http://schema.org/AudioObjectSnapshot";
/// <https://schema.org/AudioObjectSnapshot>
pub const AUDIO_OBJECT_SNAPSHOT_IRI_HTTPS: &str = "https://schema.org/AudioObjectSnapshot";
/// <https://schema.org/AudioObjectSnapshot>
pub const AUDIO_OBJECT_SNAPSHOT_LABEL: &str = "AudioObjectSnapshot";
pub struct AudioObjectSnapshotIri;
impl PartialEq<&str> for AudioObjectSnapshotIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUDIO_OBJECT_SNAPSHOT_IRI_HTTP || *other == AUDIO_OBJECT_SNAPSHOT_IRI_HTTPS
	}
}
impl PartialEq<AudioObjectSnapshotIri> for &str {
	fn eq(&self, other: &AudioObjectSnapshotIri) -> bool {
		*self == AUDIO_OBJECT_SNAPSHOT_IRI_HTTP || *self == AUDIO_OBJECT_SNAPSHOT_IRI_HTTPS
	}
}
pub struct AudioObjectSnapshotIriOrLabel;
impl PartialEq<&str> for AudioObjectSnapshotIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AudioObjectSnapshotIri || *other == AUDIO_OBJECT_SNAPSHOT_LABEL
	}
}
impl PartialEq<AudioObjectSnapshotIriOrLabel> for &str {
	fn eq(&self, other: &AudioObjectSnapshotIriOrLabel) -> bool {
		*self == AudioObjectSnapshotIri || *self == AUDIO_OBJECT_SNAPSHOT_LABEL
	}
}
