/// <https://schema.org/articleSection>
pub const ARTICLE_SECTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/articleSection";
/// <https://schema.org/articleSection>
pub const ARTICLE_SECTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/articleSection";
/// <https://schema.org/articleSection>
pub const ARTICLE_SECTION_PROPERTY_LABEL: &str = "articleSection";
pub struct ArticleSectionPropertyIri;
impl PartialEq<&str> for ArticleSectionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARTICLE_SECTION_PROPERTY_IRI_HTTP || *other == ARTICLE_SECTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArticleSectionPropertyIri> for &str {
	fn eq(&self, other: &ArticleSectionPropertyIri) -> bool {
		*self == ARTICLE_SECTION_PROPERTY_IRI_HTTP || *self == ARTICLE_SECTION_PROPERTY_IRI_HTTPS
	}
}
pub struct ArticleSectionPropertyIriOrLabel;
impl PartialEq<&str> for ArticleSectionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArticleSectionPropertyIri || *other == ARTICLE_SECTION_PROPERTY_LABEL
	}
}
impl PartialEq<ArticleSectionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArticleSectionPropertyIriOrLabel) -> bool {
		*self == ArticleSectionPropertyIri || *self == ARTICLE_SECTION_PROPERTY_LABEL
	}
}
