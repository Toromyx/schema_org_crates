/// <https://schema.org/UpdateAction>
pub const UPDATE_ACTION_IRI_HTTP: &str = "http://schema.org/UpdateAction";
/// <https://schema.org/UpdateAction>
pub const UPDATE_ACTION_IRI_HTTPS: &str = "https://schema.org/UpdateAction";
/// <https://schema.org/UpdateAction>
pub const UPDATE_ACTION_LABEL: &str = "UpdateAction";
pub struct UpdateActionIri;
impl PartialEq<&str> for UpdateActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UPDATE_ACTION_IRI_HTTP || *other == UPDATE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<UpdateActionIri> for &str {
	fn eq(&self, other: &UpdateActionIri) -> bool {
		*self == UPDATE_ACTION_IRI_HTTP || *self == UPDATE_ACTION_IRI_HTTPS
	}
}
pub struct UpdateActionIriOrLabel;
impl PartialEq<&str> for UpdateActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UpdateActionIri || *other == UPDATE_ACTION_LABEL
	}
}
impl PartialEq<UpdateActionIriOrLabel> for &str {
	fn eq(&self, other: &UpdateActionIriOrLabel) -> bool {
		*self == UpdateActionIri || *self == UPDATE_ACTION_LABEL
	}
}
