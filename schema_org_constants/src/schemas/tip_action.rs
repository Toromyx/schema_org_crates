/// <https://schema.org/TipAction>
pub const TIP_ACTION_IRI_HTTP: &str = "http://schema.org/TipAction";
/// <https://schema.org/TipAction>
pub const TIP_ACTION_IRI_HTTPS: &str = "https://schema.org/TipAction";
/// <https://schema.org/TipAction>
pub const TIP_ACTION_LABEL: &str = "TipAction";
pub struct TipActionIri;
impl PartialEq<&str> for TipActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TIP_ACTION_IRI_HTTP || *other == TIP_ACTION_IRI_HTTPS
	}
}
impl PartialEq<TipActionIri> for &str {
	fn eq(&self, other: &TipActionIri) -> bool {
		*self == TIP_ACTION_IRI_HTTP || *self == TIP_ACTION_IRI_HTTPS
	}
}
pub struct TipActionIriOrLabel;
impl PartialEq<&str> for TipActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TipActionIri || *other == TIP_ACTION_LABEL
	}
}
impl PartialEq<TipActionIriOrLabel> for &str {
	fn eq(&self, other: &TipActionIriOrLabel) -> bool {
		*self == TipActionIri || *self == TIP_ACTION_LABEL
	}
}
