/// <https://schema.org/AssessAction>
pub const ASSESS_ACTION_IRI_HTTP: &str = "http://schema.org/AssessAction";
/// <https://schema.org/AssessAction>
pub const ASSESS_ACTION_IRI_HTTPS: &str = "https://schema.org/AssessAction";
/// <https://schema.org/AssessAction>
pub const ASSESS_ACTION_LABEL: &str = "AssessAction";
pub struct AssessActionIri;
impl PartialEq<&str> for AssessActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSESS_ACTION_IRI_HTTP || *other == ASSESS_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AssessActionIri> for &str {
	fn eq(&self, other: &AssessActionIri) -> bool {
		*self == ASSESS_ACTION_IRI_HTTP || *self == ASSESS_ACTION_IRI_HTTPS
	}
}
pub struct AssessActionIriOrLabel;
impl PartialEq<&str> for AssessActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssessActionIri || *other == ASSESS_ACTION_LABEL
	}
}
impl PartialEq<AssessActionIriOrLabel> for &str {
	fn eq(&self, other: &AssessActionIriOrLabel) -> bool {
		*self == AssessActionIri || *self == ASSESS_ACTION_LABEL
	}
}
