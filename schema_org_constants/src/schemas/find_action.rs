/// <https://schema.org/FindAction>
pub const FIND_ACTION_IRI_HTTP: &str = "http://schema.org/FindAction";
/// <https://schema.org/FindAction>
pub const FIND_ACTION_IRI_HTTPS: &str = "https://schema.org/FindAction";
/// <https://schema.org/FindAction>
pub const FIND_ACTION_LABEL: &str = "FindAction";
pub struct FindActionIri;
impl PartialEq<&str> for FindActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FIND_ACTION_IRI_HTTP || *other == FIND_ACTION_IRI_HTTPS
	}
}
impl PartialEq<FindActionIri> for &str {
	fn eq(&self, other: &FindActionIri) -> bool {
		*self == FIND_ACTION_IRI_HTTP || *self == FIND_ACTION_IRI_HTTPS
	}
}
pub struct FindActionIriOrLabel;
impl PartialEq<&str> for FindActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FindActionIri || *other == FIND_ACTION_LABEL
	}
}
impl PartialEq<FindActionIriOrLabel> for &str {
	fn eq(&self, other: &FindActionIriOrLabel) -> bool {
		*self == FindActionIri || *self == FIND_ACTION_LABEL
	}
}
