/// <https://schema.org/DeleteAction>
pub const DELETE_ACTION_IRI_HTTP: &str = "http://schema.org/DeleteAction";
/// <https://schema.org/DeleteAction>
pub const DELETE_ACTION_IRI_HTTPS: &str = "https://schema.org/DeleteAction";
/// <https://schema.org/DeleteAction>
pub const DELETE_ACTION_LABEL: &str = "DeleteAction";
pub struct DeleteActionIri;
impl PartialEq<&str> for DeleteActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELETE_ACTION_IRI_HTTP || *other == DELETE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DeleteActionIri> for &str {
	fn eq(&self, other: &DeleteActionIri) -> bool {
		*self == DELETE_ACTION_IRI_HTTP || *self == DELETE_ACTION_IRI_HTTPS
	}
}
pub struct DeleteActionIriOrLabel;
impl PartialEq<&str> for DeleteActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeleteActionIri || *other == DELETE_ACTION_LABEL
	}
}
impl PartialEq<DeleteActionIriOrLabel> for &str {
	fn eq(&self, other: &DeleteActionIriOrLabel) -> bool {
		*self == DeleteActionIri || *self == DELETE_ACTION_LABEL
	}
}
