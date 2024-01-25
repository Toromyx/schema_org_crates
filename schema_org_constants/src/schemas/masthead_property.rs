/// <https://schema.org/masthead>
pub const MASTHEAD_PROPERTY_IRI_HTTP: &str = "http://schema.org/masthead";
/// <https://schema.org/masthead>
pub const MASTHEAD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/masthead";
/// <https://schema.org/masthead>
pub const MASTHEAD_PROPERTY_LABEL: &str = "masthead";
pub struct MastheadPropertyIri;
impl PartialEq<&str> for MastheadPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MASTHEAD_PROPERTY_IRI_HTTP || *other == MASTHEAD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MastheadPropertyIri> for &str {
	fn eq(&self, other: &MastheadPropertyIri) -> bool {
		*self == MASTHEAD_PROPERTY_IRI_HTTP || *self == MASTHEAD_PROPERTY_IRI_HTTPS
	}
}
pub struct MastheadPropertyIriOrLabel;
impl PartialEq<&str> for MastheadPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MastheadPropertyIri || *other == MASTHEAD_PROPERTY_LABEL
	}
}
impl PartialEq<MastheadPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MastheadPropertyIriOrLabel) -> bool {
		*self == MastheadPropertyIri || *self == MASTHEAD_PROPERTY_LABEL
	}
}
