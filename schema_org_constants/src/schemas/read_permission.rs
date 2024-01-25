/// <https://schema.org/ReadPermission>
pub const READ_PERMISSION_IRI_HTTP: &str = "http://schema.org/ReadPermission";
/// <https://schema.org/ReadPermission>
pub const READ_PERMISSION_IRI_HTTPS: &str = "https://schema.org/ReadPermission";
/// <https://schema.org/ReadPermission>
pub const READ_PERMISSION_LABEL: &str = "ReadPermission";
pub struct ReadPermissionIri;
impl PartialEq<&str> for ReadPermissionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == READ_PERMISSION_IRI_HTTP || *other == READ_PERMISSION_IRI_HTTPS
	}
}
impl PartialEq<ReadPermissionIri> for &str {
	fn eq(&self, other: &ReadPermissionIri) -> bool {
		*self == READ_PERMISSION_IRI_HTTP || *self == READ_PERMISSION_IRI_HTTPS
	}
}
pub struct ReadPermissionIriOrLabel;
impl PartialEq<&str> for ReadPermissionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReadPermissionIri || *other == READ_PERMISSION_LABEL
	}
}
impl PartialEq<ReadPermissionIriOrLabel> for &str {
	fn eq(&self, other: &ReadPermissionIriOrLabel) -> bool {
		*self == ReadPermissionIri || *self == READ_PERMISSION_LABEL
	}
}
