/// <https://schema.org/ReturnAction>
pub const RETURN_ACTION_IRI_HTTP: &str = "http://schema.org/ReturnAction";
/// <https://schema.org/ReturnAction>
pub const RETURN_ACTION_IRI_HTTPS: &str = "https://schema.org/ReturnAction";
/// <https://schema.org/ReturnAction>
pub const RETURN_ACTION_LABEL: &str = "ReturnAction";
pub struct ReturnActionIri;
impl PartialEq<&str> for ReturnActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_ACTION_IRI_HTTP || *other == RETURN_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ReturnActionIri> for &str {
	fn eq(&self, other: &ReturnActionIri) -> bool {
		*self == RETURN_ACTION_IRI_HTTP || *self == RETURN_ACTION_IRI_HTTPS
	}
}
pub struct ReturnActionIriOrLabel;
impl PartialEq<&str> for ReturnActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnActionIri || *other == RETURN_ACTION_LABEL
	}
}
impl PartialEq<ReturnActionIriOrLabel> for &str {
	fn eq(&self, other: &ReturnActionIriOrLabel) -> bool {
		*self == ReturnActionIri || *self == RETURN_ACTION_LABEL
	}
}
