/// <https://schema.org/VideoObjectSnapshot>
pub const VIDEO_OBJECT_SNAPSHOT_IRI_HTTP: &str = "http://schema.org/VideoObjectSnapshot";
/// <https://schema.org/VideoObjectSnapshot>
pub const VIDEO_OBJECT_SNAPSHOT_IRI_HTTPS: &str = "https://schema.org/VideoObjectSnapshot";
/// <https://schema.org/VideoObjectSnapshot>
pub const VIDEO_OBJECT_SNAPSHOT_LABEL: &str = "VideoObjectSnapshot";
pub struct VideoObjectSnapshotIri;
impl PartialEq<&str> for VideoObjectSnapshotIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_OBJECT_SNAPSHOT_IRI_HTTP || *other == VIDEO_OBJECT_SNAPSHOT_IRI_HTTPS
	}
}
impl PartialEq<VideoObjectSnapshotIri> for &str {
	fn eq(&self, other: &VideoObjectSnapshotIri) -> bool {
		*self == VIDEO_OBJECT_SNAPSHOT_IRI_HTTP || *self == VIDEO_OBJECT_SNAPSHOT_IRI_HTTPS
	}
}
pub struct VideoObjectSnapshotIriOrLabel;
impl PartialEq<&str> for VideoObjectSnapshotIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoObjectSnapshotIri || *other == VIDEO_OBJECT_SNAPSHOT_LABEL
	}
}
impl PartialEq<VideoObjectSnapshotIriOrLabel> for &str {
	fn eq(&self, other: &VideoObjectSnapshotIriOrLabel) -> bool {
		*self == VideoObjectSnapshotIri || *self == VIDEO_OBJECT_SNAPSHOT_LABEL
	}
}
