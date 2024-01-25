/// <https://schema.org/Terminated>
pub const TERMINATED_IRI_HTTP: &str = "http://schema.org/Terminated";
/// <https://schema.org/Terminated>
pub const TERMINATED_IRI_HTTPS: &str = "https://schema.org/Terminated";
/// <https://schema.org/Terminated>
pub const TERMINATED_LABEL: &str = "Terminated";
pub struct TerminatedIri;
impl PartialEq<&str> for TerminatedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TERMINATED_IRI_HTTP || *other == TERMINATED_IRI_HTTPS
	}
}
impl PartialEq<TerminatedIri> for &str {
	fn eq(&self, other: &TerminatedIri) -> bool {
		*self == TERMINATED_IRI_HTTP || *self == TERMINATED_IRI_HTTPS
	}
}
pub struct TerminatedIriOrLabel;
impl PartialEq<&str> for TerminatedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TerminatedIri || *other == TERMINATED_LABEL
	}
}
impl PartialEq<TerminatedIriOrLabel> for &str {
	fn eq(&self, other: &TerminatedIriOrLabel) -> bool {
		*self == TerminatedIri || *self == TERMINATED_LABEL
	}
}
