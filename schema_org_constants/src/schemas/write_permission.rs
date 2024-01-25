/// <https://schema.org/WritePermission>
pub const WRITE_PERMISSION_IRI_HTTP: &str = "http://schema.org/WritePermission";
/// <https://schema.org/WritePermission>
pub const WRITE_PERMISSION_IRI_HTTPS: &str = "https://schema.org/WritePermission";
/// <https://schema.org/WritePermission>
pub const WRITE_PERMISSION_LABEL: &str = "WritePermission";
pub struct WritePermissionIri;
impl PartialEq<&str> for WritePermissionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WRITE_PERMISSION_IRI_HTTP || *other == WRITE_PERMISSION_IRI_HTTPS
	}
}
impl PartialEq<WritePermissionIri> for &str {
	fn eq(&self, other: &WritePermissionIri) -> bool {
		*self == WRITE_PERMISSION_IRI_HTTP || *self == WRITE_PERMISSION_IRI_HTTPS
	}
}
pub struct WritePermissionIriOrLabel;
impl PartialEq<&str> for WritePermissionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WritePermissionIri || *other == WRITE_PERMISSION_LABEL
	}
}
impl PartialEq<WritePermissionIriOrLabel> for &str {
	fn eq(&self, other: &WritePermissionIriOrLabel) -> bool {
		*self == WritePermissionIri || *self == WRITE_PERMISSION_LABEL
	}
}
