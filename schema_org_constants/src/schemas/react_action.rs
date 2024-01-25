/// <https://schema.org/ReactAction>
pub const REACT_ACTION_IRI_HTTP: &str = "http://schema.org/ReactAction";
/// <https://schema.org/ReactAction>
pub const REACT_ACTION_IRI_HTTPS: &str = "https://schema.org/ReactAction";
/// <https://schema.org/ReactAction>
pub const REACT_ACTION_LABEL: &str = "ReactAction";
pub struct ReactActionIri;
impl PartialEq<&str> for ReactActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REACT_ACTION_IRI_HTTP || *other == REACT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ReactActionIri> for &str {
	fn eq(&self, other: &ReactActionIri) -> bool {
		*self == REACT_ACTION_IRI_HTTP || *self == REACT_ACTION_IRI_HTTPS
	}
}
pub struct ReactActionIriOrLabel;
impl PartialEq<&str> for ReactActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReactActionIri || *other == REACT_ACTION_LABEL
	}
}
impl PartialEq<ReactActionIriOrLabel> for &str {
	fn eq(&self, other: &ReactActionIriOrLabel) -> bool {
		*self == ReactActionIri || *self == REACT_ACTION_LABEL
	}
}
