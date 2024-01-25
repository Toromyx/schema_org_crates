/// <https://schema.org/ActiveActionStatus>
pub const ACTIVE_ACTION_STATUS_IRI_HTTP: &str = "http://schema.org/ActiveActionStatus";
/// <https://schema.org/ActiveActionStatus>
pub const ACTIVE_ACTION_STATUS_IRI_HTTPS: &str = "https://schema.org/ActiveActionStatus";
/// <https://schema.org/ActiveActionStatus>
pub const ACTIVE_ACTION_STATUS_LABEL: &str = "ActiveActionStatus";
pub struct ActiveActionStatusIri;
impl PartialEq<&str> for ActiveActionStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTIVE_ACTION_STATUS_IRI_HTTP || *other == ACTIVE_ACTION_STATUS_IRI_HTTPS
	}
}
impl PartialEq<ActiveActionStatusIri> for &str {
	fn eq(&self, other: &ActiveActionStatusIri) -> bool {
		*self == ACTIVE_ACTION_STATUS_IRI_HTTP || *self == ACTIVE_ACTION_STATUS_IRI_HTTPS
	}
}
pub struct ActiveActionStatusIriOrLabel;
impl PartialEq<&str> for ActiveActionStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActiveActionStatusIri || *other == ACTIVE_ACTION_STATUS_LABEL
	}
}
impl PartialEq<ActiveActionStatusIriOrLabel> for &str {
	fn eq(&self, other: &ActiveActionStatusIriOrLabel) -> bool {
		*self == ActiveActionStatusIri || *self == ACTIVE_ACTION_STATUS_LABEL
	}
}
