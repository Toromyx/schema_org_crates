/// <https://schema.org/CommentPermission>
pub const COMMENT_PERMISSION_IRI_HTTP: &str = "http://schema.org/CommentPermission";
/// <https://schema.org/CommentPermission>
pub const COMMENT_PERMISSION_IRI_HTTPS: &str = "https://schema.org/CommentPermission";
/// <https://schema.org/CommentPermission>
pub const COMMENT_PERMISSION_LABEL: &str = "CommentPermission";
pub struct CommentPermissionIri;
impl PartialEq<&str> for CommentPermissionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMMENT_PERMISSION_IRI_HTTP || *other == COMMENT_PERMISSION_IRI_HTTPS
	}
}
impl PartialEq<CommentPermissionIri> for &str {
	fn eq(&self, other: &CommentPermissionIri) -> bool {
		*self == COMMENT_PERMISSION_IRI_HTTP || *self == COMMENT_PERMISSION_IRI_HTTPS
	}
}
pub struct CommentPermissionIriOrLabel;
impl PartialEq<&str> for CommentPermissionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CommentPermissionIri || *other == COMMENT_PERMISSION_LABEL
	}
}
impl PartialEq<CommentPermissionIriOrLabel> for &str {
	fn eq(&self, other: &CommentPermissionIriOrLabel) -> bool {
		*self == CommentPermissionIri || *self == COMMENT_PERMISSION_LABEL
	}
}
