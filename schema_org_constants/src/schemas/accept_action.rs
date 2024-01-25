/// <https://schema.org/AcceptAction>
pub const ACCEPT_ACTION_IRI_HTTP: &str = "http://schema.org/AcceptAction";
/// <https://schema.org/AcceptAction>
pub const ACCEPT_ACTION_IRI_HTTPS: &str = "https://schema.org/AcceptAction";
/// <https://schema.org/AcceptAction>
pub const ACCEPT_ACTION_LABEL: &str = "AcceptAction";
pub struct AcceptActionIri;
impl PartialEq<&str> for AcceptActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCEPT_ACTION_IRI_HTTP || *other == ACCEPT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AcceptActionIri> for &str {
	fn eq(&self, other: &AcceptActionIri) -> bool {
		*self == ACCEPT_ACTION_IRI_HTTP || *self == ACCEPT_ACTION_IRI_HTTPS
	}
}
pub struct AcceptActionIriOrLabel;
impl PartialEq<&str> for AcceptActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AcceptActionIri || *other == ACCEPT_ACTION_LABEL
	}
}
impl PartialEq<AcceptActionIriOrLabel> for &str {
	fn eq(&self, other: &AcceptActionIriOrLabel) -> bool {
		*self == AcceptActionIri || *self == ACCEPT_ACTION_LABEL
	}
}
