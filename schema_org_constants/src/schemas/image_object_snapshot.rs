/// <https://schema.org/ImageObjectSnapshot>
pub const IMAGE_OBJECT_SNAPSHOT_IRI_HTTP: &str = "http://schema.org/ImageObjectSnapshot";
/// <https://schema.org/ImageObjectSnapshot>
pub const IMAGE_OBJECT_SNAPSHOT_IRI_HTTPS: &str = "https://schema.org/ImageObjectSnapshot";
/// <https://schema.org/ImageObjectSnapshot>
pub const IMAGE_OBJECT_SNAPSHOT_LABEL: &str = "ImageObjectSnapshot";
pub struct ImageObjectSnapshotIri;
impl PartialEq<&str> for ImageObjectSnapshotIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IMAGE_OBJECT_SNAPSHOT_IRI_HTTP || *other == IMAGE_OBJECT_SNAPSHOT_IRI_HTTPS
	}
}
impl PartialEq<ImageObjectSnapshotIri> for &str {
	fn eq(&self, other: &ImageObjectSnapshotIri) -> bool {
		*self == IMAGE_OBJECT_SNAPSHOT_IRI_HTTP || *self == IMAGE_OBJECT_SNAPSHOT_IRI_HTTPS
	}
}
pub struct ImageObjectSnapshotIriOrLabel;
impl PartialEq<&str> for ImageObjectSnapshotIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ImageObjectSnapshotIri || *other == IMAGE_OBJECT_SNAPSHOT_LABEL
	}
}
impl PartialEq<ImageObjectSnapshotIriOrLabel> for &str {
	fn eq(&self, other: &ImageObjectSnapshotIriOrLabel) -> bool {
		*self == ImageObjectSnapshotIri || *self == IMAGE_OBJECT_SNAPSHOT_LABEL
	}
}
