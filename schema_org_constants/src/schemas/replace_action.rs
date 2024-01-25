/// <https://schema.org/ReplaceAction>
pub const REPLACE_ACTION_IRI_HTTP: &str = "http://schema.org/ReplaceAction";
/// <https://schema.org/ReplaceAction>
pub const REPLACE_ACTION_IRI_HTTPS: &str = "https://schema.org/ReplaceAction";
/// <https://schema.org/ReplaceAction>
pub const REPLACE_ACTION_LABEL: &str = "ReplaceAction";
pub struct ReplaceActionIri;
impl PartialEq<&str> for ReplaceActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPLACE_ACTION_IRI_HTTP || *other == REPLACE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ReplaceActionIri> for &str {
	fn eq(&self, other: &ReplaceActionIri) -> bool {
		*self == REPLACE_ACTION_IRI_HTTP || *self == REPLACE_ACTION_IRI_HTTPS
	}
}
pub struct ReplaceActionIriOrLabel;
impl PartialEq<&str> for ReplaceActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReplaceActionIri || *other == REPLACE_ACTION_LABEL
	}
}
impl PartialEq<ReplaceActionIriOrLabel> for &str {
	fn eq(&self, other: &ReplaceActionIriOrLabel) -> bool {
		*self == ReplaceActionIri || *self == REPLACE_ACTION_LABEL
	}
}
