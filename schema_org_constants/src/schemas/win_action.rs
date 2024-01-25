/// <https://schema.org/WinAction>
pub const WIN_ACTION_IRI_HTTP: &str = "http://schema.org/WinAction";
/// <https://schema.org/WinAction>
pub const WIN_ACTION_IRI_HTTPS: &str = "https://schema.org/WinAction";
/// <https://schema.org/WinAction>
pub const WIN_ACTION_LABEL: &str = "WinAction";
pub struct WinActionIri;
impl PartialEq<&str> for WinActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WIN_ACTION_IRI_HTTP || *other == WIN_ACTION_IRI_HTTPS
	}
}
impl PartialEq<WinActionIri> for &str {
	fn eq(&self, other: &WinActionIri) -> bool {
		*self == WIN_ACTION_IRI_HTTP || *self == WIN_ACTION_IRI_HTTPS
	}
}
pub struct WinActionIriOrLabel;
impl PartialEq<&str> for WinActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WinActionIri || *other == WIN_ACTION_LABEL
	}
}
impl PartialEq<WinActionIriOrLabel> for &str {
	fn eq(&self, other: &WinActionIriOrLabel) -> bool {
		*self == WinActionIri || *self == WIN_ACTION_LABEL
	}
}
