/// <https://schema.org/about>
pub const ABOUT_PROPERTY_IRI_HTTP: &str = "http://schema.org/about";
/// <https://schema.org/about>
pub const ABOUT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/about";
/// <https://schema.org/about>
pub const ABOUT_PROPERTY_LABEL: &str = "about";
pub struct AboutPropertyIri;
impl PartialEq<&str> for AboutPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ABOUT_PROPERTY_IRI_HTTP || *other == ABOUT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AboutPropertyIri> for &str {
	fn eq(&self, other: &AboutPropertyIri) -> bool {
		*self == ABOUT_PROPERTY_IRI_HTTP || *self == ABOUT_PROPERTY_IRI_HTTPS
	}
}
pub struct AboutPropertyIriOrLabel;
impl PartialEq<&str> for AboutPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AboutPropertyIri || *other == ABOUT_PROPERTY_LABEL
	}
}
impl PartialEq<AboutPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AboutPropertyIriOrLabel) -> bool {
		*self == AboutPropertyIri || *self == ABOUT_PROPERTY_LABEL
	}
}
