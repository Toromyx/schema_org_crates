/// <https://schema.org/MarryAction>
pub const MARRY_ACTION_IRI_HTTP: &str = "http://schema.org/MarryAction";
/// <https://schema.org/MarryAction>
pub const MARRY_ACTION_IRI_HTTPS: &str = "https://schema.org/MarryAction";
/// <https://schema.org/MarryAction>
pub const MARRY_ACTION_LABEL: &str = "MarryAction";
pub struct MarryActionIri;
impl PartialEq<&str> for MarryActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MARRY_ACTION_IRI_HTTP || *other == MARRY_ACTION_IRI_HTTPS
	}
}
impl PartialEq<MarryActionIri> for &str {
	fn eq(&self, other: &MarryActionIri) -> bool {
		*self == MARRY_ACTION_IRI_HTTP || *self == MARRY_ACTION_IRI_HTTPS
	}
}
pub struct MarryActionIriOrLabel;
impl PartialEq<&str> for MarryActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MarryActionIri || *other == MARRY_ACTION_LABEL
	}
}
impl PartialEq<MarryActionIriOrLabel> for &str {
	fn eq(&self, other: &MarryActionIriOrLabel) -> bool {
		*self == MarryActionIri || *self == MARRY_ACTION_LABEL
	}
}
