/// <https://schema.org/CompletedActionStatus>
pub const COMPLETED_ACTION_STATUS_IRI_HTTP: &str = "http://schema.org/CompletedActionStatus";
/// <https://schema.org/CompletedActionStatus>
pub const COMPLETED_ACTION_STATUS_IRI_HTTPS: &str = "https://schema.org/CompletedActionStatus";
/// <https://schema.org/CompletedActionStatus>
pub const COMPLETED_ACTION_STATUS_LABEL: &str = "CompletedActionStatus";
pub struct CompletedActionStatusIri;
impl PartialEq<&str> for CompletedActionStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPLETED_ACTION_STATUS_IRI_HTTP || *other == COMPLETED_ACTION_STATUS_IRI_HTTPS
	}
}
impl PartialEq<CompletedActionStatusIri> for &str {
	fn eq(&self, other: &CompletedActionStatusIri) -> bool {
		*self == COMPLETED_ACTION_STATUS_IRI_HTTP || *self == COMPLETED_ACTION_STATUS_IRI_HTTPS
	}
}
pub struct CompletedActionStatusIriOrLabel;
impl PartialEq<&str> for CompletedActionStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompletedActionStatusIri || *other == COMPLETED_ACTION_STATUS_LABEL
	}
}
impl PartialEq<CompletedActionStatusIriOrLabel> for &str {
	fn eq(&self, other: &CompletedActionStatusIriOrLabel) -> bool {
		*self == CompletedActionStatusIri || *self == COMPLETED_ACTION_STATUS_LABEL
	}
}
