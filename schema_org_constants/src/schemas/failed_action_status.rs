/// <https://schema.org/FailedActionStatus>
pub const FAILED_ACTION_STATUS_IRI_HTTP: &str = "http://schema.org/FailedActionStatus";
/// <https://schema.org/FailedActionStatus>
pub const FAILED_ACTION_STATUS_IRI_HTTPS: &str = "https://schema.org/FailedActionStatus";
/// <https://schema.org/FailedActionStatus>
pub const FAILED_ACTION_STATUS_LABEL: &str = "FailedActionStatus";
pub struct FailedActionStatusIri;
impl PartialEq<&str> for FailedActionStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FAILED_ACTION_STATUS_IRI_HTTP || *other == FAILED_ACTION_STATUS_IRI_HTTPS
	}
}
impl PartialEq<FailedActionStatusIri> for &str {
	fn eq(&self, other: &FailedActionStatusIri) -> bool {
		*self == FAILED_ACTION_STATUS_IRI_HTTP || *self == FAILED_ACTION_STATUS_IRI_HTTPS
	}
}
pub struct FailedActionStatusIriOrLabel;
impl PartialEq<&str> for FailedActionStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FailedActionStatusIri || *other == FAILED_ACTION_STATUS_LABEL
	}
}
impl PartialEq<FailedActionStatusIriOrLabel> for &str {
	fn eq(&self, other: &FailedActionStatusIriOrLabel) -> bool {
		*self == FailedActionStatusIri || *self == FAILED_ACTION_STATUS_LABEL
	}
}
