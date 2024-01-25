/// <https://schema.org/JoinAction>
pub const JOIN_ACTION_IRI_HTTP: &str = "http://schema.org/JoinAction";
/// <https://schema.org/JoinAction>
pub const JOIN_ACTION_IRI_HTTPS: &str = "https://schema.org/JoinAction";
/// <https://schema.org/JoinAction>
pub const JOIN_ACTION_LABEL: &str = "JoinAction";
pub struct JoinActionIri;
impl PartialEq<&str> for JoinActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JOIN_ACTION_IRI_HTTP || *other == JOIN_ACTION_IRI_HTTPS
	}
}
impl PartialEq<JoinActionIri> for &str {
	fn eq(&self, other: &JoinActionIri) -> bool {
		*self == JOIN_ACTION_IRI_HTTP || *self == JOIN_ACTION_IRI_HTTPS
	}
}
pub struct JoinActionIriOrLabel;
impl PartialEq<&str> for JoinActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JoinActionIri || *other == JOIN_ACTION_LABEL
	}
}
impl PartialEq<JoinActionIriOrLabel> for &str {
	fn eq(&self, other: &JoinActionIriOrLabel) -> bool {
		*self == JoinActionIri || *self == JOIN_ACTION_LABEL
	}
}
