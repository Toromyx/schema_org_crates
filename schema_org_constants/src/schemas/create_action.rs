/// <https://schema.org/CreateAction>
pub const CREATE_ACTION_IRI_HTTP: &str = "http://schema.org/CreateAction";
/// <https://schema.org/CreateAction>
pub const CREATE_ACTION_IRI_HTTPS: &str = "https://schema.org/CreateAction";
/// <https://schema.org/CreateAction>
pub const CREATE_ACTION_LABEL: &str = "CreateAction";
pub struct CreateActionIri;
impl PartialEq<&str> for CreateActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREATE_ACTION_IRI_HTTP || *other == CREATE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<CreateActionIri> for &str {
	fn eq(&self, other: &CreateActionIri) -> bool {
		*self == CREATE_ACTION_IRI_HTTP || *self == CREATE_ACTION_IRI_HTTPS
	}
}
pub struct CreateActionIriOrLabel;
impl PartialEq<&str> for CreateActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CreateActionIri || *other == CREATE_ACTION_LABEL
	}
}
impl PartialEq<CreateActionIriOrLabel> for &str {
	fn eq(&self, other: &CreateActionIriOrLabel) -> bool {
		*self == CreateActionIri || *self == CREATE_ACTION_LABEL
	}
}
