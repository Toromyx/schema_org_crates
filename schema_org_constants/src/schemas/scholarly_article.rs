/// <https://schema.org/ScholarlyArticle>
pub const SCHOLARLY_ARTICLE_IRI_HTTP: &str = "http://schema.org/ScholarlyArticle";
/// <https://schema.org/ScholarlyArticle>
pub const SCHOLARLY_ARTICLE_IRI_HTTPS: &str = "https://schema.org/ScholarlyArticle";
/// <https://schema.org/ScholarlyArticle>
pub const SCHOLARLY_ARTICLE_LABEL: &str = "ScholarlyArticle";
pub struct ScholarlyArticleIri;
impl PartialEq<&str> for ScholarlyArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHOLARLY_ARTICLE_IRI_HTTP || *other == SCHOLARLY_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<ScholarlyArticleIri> for &str {
	fn eq(&self, other: &ScholarlyArticleIri) -> bool {
		*self == SCHOLARLY_ARTICLE_IRI_HTTP || *self == SCHOLARLY_ARTICLE_IRI_HTTPS
	}
}
pub struct ScholarlyArticleIriOrLabel;
impl PartialEq<&str> for ScholarlyArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScholarlyArticleIri || *other == SCHOLARLY_ARTICLE_LABEL
	}
}
impl PartialEq<ScholarlyArticleIriOrLabel> for &str {
	fn eq(&self, other: &ScholarlyArticleIriOrLabel) -> bool {
		*self == ScholarlyArticleIri || *self == SCHOLARLY_ARTICLE_LABEL
	}
}
