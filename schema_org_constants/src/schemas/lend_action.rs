/// <https://schema.org/LendAction>
pub const LEND_ACTION_IRI_HTTP: &str = "http://schema.org/LendAction";
/// <https://schema.org/LendAction>
pub const LEND_ACTION_IRI_HTTPS: &str = "https://schema.org/LendAction";
/// <https://schema.org/LendAction>
pub const LEND_ACTION_LABEL: &str = "LendAction";
pub struct LendActionIri;
impl PartialEq<&str> for LendActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEND_ACTION_IRI_HTTP || *other == LEND_ACTION_IRI_HTTPS
	}
}
impl PartialEq<LendActionIri> for &str {
	fn eq(&self, other: &LendActionIri) -> bool {
		*self == LEND_ACTION_IRI_HTTP || *self == LEND_ACTION_IRI_HTTPS
	}
}
pub struct LendActionIriOrLabel;
impl PartialEq<&str> for LendActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LendActionIri || *other == LEND_ACTION_LABEL
	}
}
impl PartialEq<LendActionIriOrLabel> for &str {
	fn eq(&self, other: &LendActionIriOrLabel) -> bool {
		*self == LendActionIri || *self == LEND_ACTION_LABEL
	}
}
