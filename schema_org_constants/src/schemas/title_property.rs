/// <https://schema.org/title>
pub const TITLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/title";
/// <https://schema.org/title>
pub const TITLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/title";
/// <https://schema.org/title>
pub const TITLE_PROPERTY_LABEL: &str = "title";
pub struct TitlePropertyIri;
impl PartialEq<&str> for TitlePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TITLE_PROPERTY_IRI_HTTP || *other == TITLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TitlePropertyIri> for &str {
	fn eq(&self, other: &TitlePropertyIri) -> bool {
		*self == TITLE_PROPERTY_IRI_HTTP || *self == TITLE_PROPERTY_IRI_HTTPS
	}
}
pub struct TitlePropertyIriOrLabel;
impl PartialEq<&str> for TitlePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TitlePropertyIri || *other == TITLE_PROPERTY_LABEL
	}
}
impl PartialEq<TitlePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TitlePropertyIriOrLabel) -> bool {
		*self == TitlePropertyIri || *self == TITLE_PROPERTY_LABEL
	}
}
