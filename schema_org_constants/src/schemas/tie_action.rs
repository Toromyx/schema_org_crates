/// <https://schema.org/TieAction>
pub const TIE_ACTION_IRI_HTTP: &str = "http://schema.org/TieAction";
/// <https://schema.org/TieAction>
pub const TIE_ACTION_IRI_HTTPS: &str = "https://schema.org/TieAction";
/// <https://schema.org/TieAction>
pub const TIE_ACTION_LABEL: &str = "TieAction";
pub struct TieActionIri;
impl PartialEq<&str> for TieActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TIE_ACTION_IRI_HTTP || *other == TIE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<TieActionIri> for &str {
	fn eq(&self, other: &TieActionIri) -> bool {
		*self == TIE_ACTION_IRI_HTTP || *self == TIE_ACTION_IRI_HTTPS
	}
}
pub struct TieActionIriOrLabel;
impl PartialEq<&str> for TieActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TieActionIri || *other == TIE_ACTION_LABEL
	}
}
impl PartialEq<TieActionIriOrLabel> for &str {
	fn eq(&self, other: &TieActionIriOrLabel) -> bool {
		*self == TieActionIri || *self == TIE_ACTION_LABEL
	}
}
