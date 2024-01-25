/// <https://schema.org/ApplyAction>
pub const APPLY_ACTION_IRI_HTTP: &str = "http://schema.org/ApplyAction";
/// <https://schema.org/ApplyAction>
pub const APPLY_ACTION_IRI_HTTPS: &str = "https://schema.org/ApplyAction";
/// <https://schema.org/ApplyAction>
pub const APPLY_ACTION_LABEL: &str = "ApplyAction";
pub struct ApplyActionIri;
impl PartialEq<&str> for ApplyActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLY_ACTION_IRI_HTTP || *other == APPLY_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ApplyActionIri> for &str {
	fn eq(&self, other: &ApplyActionIri) -> bool {
		*self == APPLY_ACTION_IRI_HTTP || *self == APPLY_ACTION_IRI_HTTPS
	}
}
pub struct ApplyActionIriOrLabel;
impl PartialEq<&str> for ApplyActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApplyActionIri || *other == APPLY_ACTION_LABEL
	}
}
impl PartialEq<ApplyActionIriOrLabel> for &str {
	fn eq(&self, other: &ApplyActionIriOrLabel) -> bool {
		*self == ApplyActionIri || *self == APPLY_ACTION_LABEL
	}
}
