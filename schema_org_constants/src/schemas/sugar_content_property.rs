/// <https://schema.org/sugarContent>
pub const SUGAR_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/sugarContent";
/// <https://schema.org/sugarContent>
pub const SUGAR_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sugarContent";
/// <https://schema.org/sugarContent>
pub const SUGAR_CONTENT_PROPERTY_LABEL: &str = "sugarContent";
pub struct SugarContentPropertyIri;
impl PartialEq<&str> for SugarContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUGAR_CONTENT_PROPERTY_IRI_HTTP || *other == SUGAR_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SugarContentPropertyIri> for &str {
	fn eq(&self, other: &SugarContentPropertyIri) -> bool {
		*self == SUGAR_CONTENT_PROPERTY_IRI_HTTP || *self == SUGAR_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct SugarContentPropertyIriOrLabel;
impl PartialEq<&str> for SugarContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SugarContentPropertyIri || *other == SUGAR_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<SugarContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SugarContentPropertyIriOrLabel) -> bool {
		*self == SugarContentPropertyIri || *self == SUGAR_CONTENT_PROPERTY_LABEL
	}
}
