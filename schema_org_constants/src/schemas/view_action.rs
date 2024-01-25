/// <https://schema.org/ViewAction>
pub const VIEW_ACTION_IRI_HTTP: &str = "http://schema.org/ViewAction";
/// <https://schema.org/ViewAction>
pub const VIEW_ACTION_IRI_HTTPS: &str = "https://schema.org/ViewAction";
/// <https://schema.org/ViewAction>
pub const VIEW_ACTION_LABEL: &str = "ViewAction";
pub struct ViewActionIri;
impl PartialEq<&str> for ViewActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIEW_ACTION_IRI_HTTP || *other == VIEW_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ViewActionIri> for &str {
	fn eq(&self, other: &ViewActionIri) -> bool {
		*self == VIEW_ACTION_IRI_HTTP || *self == VIEW_ACTION_IRI_HTTPS
	}
}
pub struct ViewActionIriOrLabel;
impl PartialEq<&str> for ViewActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ViewActionIri || *other == VIEW_ACTION_LABEL
	}
}
impl PartialEq<ViewActionIriOrLabel> for &str {
	fn eq(&self, other: &ViewActionIriOrLabel) -> bool {
		*self == ViewActionIri || *self == VIEW_ACTION_LABEL
	}
}
