/// <https://schema.org/colleagues>
#[deprecated = "This schema is superseded by <https://schema.org/colleague>."]
pub const COLLEAGUES_PROPERTY_IRI_HTTP: &str = "http://schema.org/colleagues";
/// <https://schema.org/colleagues>
#[deprecated = "This schema is superseded by <https://schema.org/colleague>."]
pub const COLLEAGUES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/colleagues";
/// <https://schema.org/colleagues>
#[deprecated = "This schema is superseded by <https://schema.org/colleague>."]
pub const COLLEAGUES_PROPERTY_LABEL: &str = "colleagues";
pub struct ColleaguesPropertyIri;
impl PartialEq<&str> for ColleaguesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLLEAGUES_PROPERTY_IRI_HTTP || *other == COLLEAGUES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ColleaguesPropertyIri> for &str {
	fn eq(&self, other: &ColleaguesPropertyIri) -> bool {
		*self == COLLEAGUES_PROPERTY_IRI_HTTP || *self == COLLEAGUES_PROPERTY_IRI_HTTPS
	}
}
pub struct ColleaguesPropertyIriOrLabel;
impl PartialEq<&str> for ColleaguesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ColleaguesPropertyIri || *other == COLLEAGUES_PROPERTY_LABEL
	}
}
impl PartialEq<ColleaguesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ColleaguesPropertyIriOrLabel) -> bool {
		*self == ColleaguesPropertyIri || *self == COLLEAGUES_PROPERTY_LABEL
	}
}
