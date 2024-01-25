/// <https://schema.org/knowsAbout>
pub const KNOWS_ABOUT_PROPERTY_IRI_HTTP: &str = "http://schema.org/knowsAbout";
/// <https://schema.org/knowsAbout>
pub const KNOWS_ABOUT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/knowsAbout";
/// <https://schema.org/knowsAbout>
pub const KNOWS_ABOUT_PROPERTY_LABEL: &str = "knowsAbout";
pub struct KnowsAboutPropertyIri;
impl PartialEq<&str> for KnowsAboutPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == KNOWS_ABOUT_PROPERTY_IRI_HTTP || *other == KNOWS_ABOUT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<KnowsAboutPropertyIri> for &str {
	fn eq(&self, other: &KnowsAboutPropertyIri) -> bool {
		*self == KNOWS_ABOUT_PROPERTY_IRI_HTTP || *self == KNOWS_ABOUT_PROPERTY_IRI_HTTPS
	}
}
pub struct KnowsAboutPropertyIriOrLabel;
impl PartialEq<&str> for KnowsAboutPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == KnowsAboutPropertyIri || *other == KNOWS_ABOUT_PROPERTY_LABEL
	}
}
impl PartialEq<KnowsAboutPropertyIriOrLabel> for &str {
	fn eq(&self, other: &KnowsAboutPropertyIriOrLabel) -> bool {
		*self == KnowsAboutPropertyIri || *self == KNOWS_ABOUT_PROPERTY_LABEL
	}
}
