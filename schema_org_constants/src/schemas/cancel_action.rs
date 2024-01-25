/// <https://schema.org/CancelAction>
pub const CANCEL_ACTION_IRI_HTTP: &str = "http://schema.org/CancelAction";
/// <https://schema.org/CancelAction>
pub const CANCEL_ACTION_IRI_HTTPS: &str = "https://schema.org/CancelAction";
/// <https://schema.org/CancelAction>
pub const CANCEL_ACTION_LABEL: &str = "CancelAction";
pub struct CancelActionIri;
impl PartialEq<&str> for CancelActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CANCEL_ACTION_IRI_HTTP || *other == CANCEL_ACTION_IRI_HTTPS
	}
}
impl PartialEq<CancelActionIri> for &str {
	fn eq(&self, other: &CancelActionIri) -> bool {
		*self == CANCEL_ACTION_IRI_HTTP || *self == CANCEL_ACTION_IRI_HTTPS
	}
}
pub struct CancelActionIriOrLabel;
impl PartialEq<&str> for CancelActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CancelActionIri || *other == CANCEL_ACTION_LABEL
	}
}
impl PartialEq<CancelActionIriOrLabel> for &str {
	fn eq(&self, other: &CancelActionIriOrLabel) -> bool {
		*self == CancelActionIri || *self == CANCEL_ACTION_LABEL
	}
}
