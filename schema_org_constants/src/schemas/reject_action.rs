/// <https://schema.org/RejectAction>
pub const REJECT_ACTION_IRI_HTTP: &str = "http://schema.org/RejectAction";
/// <https://schema.org/RejectAction>
pub const REJECT_ACTION_IRI_HTTPS: &str = "https://schema.org/RejectAction";
/// <https://schema.org/RejectAction>
pub const REJECT_ACTION_LABEL: &str = "RejectAction";
pub struct RejectActionIri;
impl PartialEq<&str> for RejectActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REJECT_ACTION_IRI_HTTP || *other == REJECT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<RejectActionIri> for &str {
	fn eq(&self, other: &RejectActionIri) -> bool {
		*self == REJECT_ACTION_IRI_HTTP || *self == REJECT_ACTION_IRI_HTTPS
	}
}
pub struct RejectActionIriOrLabel;
impl PartialEq<&str> for RejectActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RejectActionIri || *other == REJECT_ACTION_LABEL
	}
}
impl PartialEq<RejectActionIriOrLabel> for &str {
	fn eq(&self, other: &RejectActionIriOrLabel) -> bool {
		*self == RejectActionIri || *self == REJECT_ACTION_LABEL
	}
}
