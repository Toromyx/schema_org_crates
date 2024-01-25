/// <https://schema.org/IgnoreAction>
pub const IGNORE_ACTION_IRI_HTTP: &str = "http://schema.org/IgnoreAction";
/// <https://schema.org/IgnoreAction>
pub const IGNORE_ACTION_IRI_HTTPS: &str = "https://schema.org/IgnoreAction";
/// <https://schema.org/IgnoreAction>
pub const IGNORE_ACTION_LABEL: &str = "IgnoreAction";
pub struct IgnoreActionIri;
impl PartialEq<&str> for IgnoreActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IGNORE_ACTION_IRI_HTTP || *other == IGNORE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<IgnoreActionIri> for &str {
	fn eq(&self, other: &IgnoreActionIri) -> bool {
		*self == IGNORE_ACTION_IRI_HTTP || *self == IGNORE_ACTION_IRI_HTTPS
	}
}
pub struct IgnoreActionIriOrLabel;
impl PartialEq<&str> for IgnoreActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IgnoreActionIri || *other == IGNORE_ACTION_LABEL
	}
}
impl PartialEq<IgnoreActionIriOrLabel> for &str {
	fn eq(&self, other: &IgnoreActionIriOrLabel) -> bool {
		*self == IgnoreActionIri || *self == IGNORE_ACTION_LABEL
	}
}
