/// <https://schema.org/WantAction>
pub const WANT_ACTION_IRI_HTTP: &str = "http://schema.org/WantAction";
/// <https://schema.org/WantAction>
pub const WANT_ACTION_IRI_HTTPS: &str = "https://schema.org/WantAction";
/// <https://schema.org/WantAction>
pub const WANT_ACTION_LABEL: &str = "WantAction";
pub struct WantActionIri;
impl PartialEq<&str> for WantActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WANT_ACTION_IRI_HTTP || *other == WANT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<WantActionIri> for &str {
	fn eq(&self, other: &WantActionIri) -> bool {
		*self == WANT_ACTION_IRI_HTTP || *self == WANT_ACTION_IRI_HTTPS
	}
}
pub struct WantActionIriOrLabel;
impl PartialEq<&str> for WantActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WantActionIri || *other == WANT_ACTION_LABEL
	}
}
impl PartialEq<WantActionIriOrLabel> for &str {
	fn eq(&self, other: &WantActionIriOrLabel) -> bool {
		*self == WantActionIri || *self == WANT_ACTION_LABEL
	}
}
