/// <https://schema.org/winner>
pub const WINNER_PROPERTY_IRI_HTTP: &str = "http://schema.org/winner";
/// <https://schema.org/winner>
pub const WINNER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/winner";
/// <https://schema.org/winner>
pub const WINNER_PROPERTY_LABEL: &str = "winner";
pub struct WinnerPropertyIri;
impl PartialEq<&str> for WinnerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WINNER_PROPERTY_IRI_HTTP || *other == WINNER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WinnerPropertyIri> for &str {
	fn eq(&self, other: &WinnerPropertyIri) -> bool {
		*self == WINNER_PROPERTY_IRI_HTTP || *self == WINNER_PROPERTY_IRI_HTTPS
	}
}
pub struct WinnerPropertyIriOrLabel;
impl PartialEq<&str> for WinnerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WinnerPropertyIri || *other == WINNER_PROPERTY_LABEL
	}
}
impl PartialEq<WinnerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WinnerPropertyIriOrLabel) -> bool {
		*self == WinnerPropertyIri || *self == WINNER_PROPERTY_LABEL
	}
}
