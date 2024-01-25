/// <https://schema.org/BefriendAction>
pub const BEFRIEND_ACTION_IRI_HTTP: &str = "http://schema.org/BefriendAction";
/// <https://schema.org/BefriendAction>
pub const BEFRIEND_ACTION_IRI_HTTPS: &str = "https://schema.org/BefriendAction";
/// <https://schema.org/BefriendAction>
pub const BEFRIEND_ACTION_LABEL: &str = "BefriendAction";
pub struct BefriendActionIri;
impl PartialEq<&str> for BefriendActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BEFRIEND_ACTION_IRI_HTTP || *other == BEFRIEND_ACTION_IRI_HTTPS
	}
}
impl PartialEq<BefriendActionIri> for &str {
	fn eq(&self, other: &BefriendActionIri) -> bool {
		*self == BEFRIEND_ACTION_IRI_HTTP || *self == BEFRIEND_ACTION_IRI_HTTPS
	}
}
pub struct BefriendActionIriOrLabel;
impl PartialEq<&str> for BefriendActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BefriendActionIri || *other == BEFRIEND_ACTION_LABEL
	}
}
impl PartialEq<BefriendActionIriOrLabel> for &str {
	fn eq(&self, other: &BefriendActionIriOrLabel) -> bool {
		*self == BefriendActionIri || *self == BEFRIEND_ACTION_LABEL
	}
}
