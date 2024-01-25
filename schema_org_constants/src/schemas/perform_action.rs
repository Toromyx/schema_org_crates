/// <https://schema.org/PerformAction>
pub const PERFORM_ACTION_IRI_HTTP: &str = "http://schema.org/PerformAction";
/// <https://schema.org/PerformAction>
pub const PERFORM_ACTION_IRI_HTTPS: &str = "https://schema.org/PerformAction";
/// <https://schema.org/PerformAction>
pub const PERFORM_ACTION_LABEL: &str = "PerformAction";
pub struct PerformActionIri;
impl PartialEq<&str> for PerformActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERFORM_ACTION_IRI_HTTP || *other == PERFORM_ACTION_IRI_HTTPS
	}
}
impl PartialEq<PerformActionIri> for &str {
	fn eq(&self, other: &PerformActionIri) -> bool {
		*self == PERFORM_ACTION_IRI_HTTP || *self == PERFORM_ACTION_IRI_HTTPS
	}
}
pub struct PerformActionIriOrLabel;
impl PartialEq<&str> for PerformActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PerformActionIri || *other == PERFORM_ACTION_LABEL
	}
}
impl PartialEq<PerformActionIriOrLabel> for &str {
	fn eq(&self, other: &PerformActionIriOrLabel) -> bool {
		*self == PerformActionIri || *self == PERFORM_ACTION_LABEL
	}
}
