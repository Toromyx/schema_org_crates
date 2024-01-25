/// <https://schema.org/TakeAction>
pub const TAKE_ACTION_IRI_HTTP: &str = "http://schema.org/TakeAction";
/// <https://schema.org/TakeAction>
pub const TAKE_ACTION_IRI_HTTPS: &str = "https://schema.org/TakeAction";
/// <https://schema.org/TakeAction>
pub const TAKE_ACTION_LABEL: &str = "TakeAction";
pub struct TakeActionIri;
impl PartialEq<&str> for TakeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAKE_ACTION_IRI_HTTP || *other == TAKE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<TakeActionIri> for &str {
	fn eq(&self, other: &TakeActionIri) -> bool {
		*self == TAKE_ACTION_IRI_HTTP || *self == TAKE_ACTION_IRI_HTTPS
	}
}
pub struct TakeActionIriOrLabel;
impl PartialEq<&str> for TakeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TakeActionIri || *other == TAKE_ACTION_LABEL
	}
}
impl PartialEq<TakeActionIriOrLabel> for &str {
	fn eq(&self, other: &TakeActionIriOrLabel) -> bool {
		*self == TakeActionIri || *self == TAKE_ACTION_LABEL
	}
}
