/// <https://schema.org/TechArticle>
pub const TECH_ARTICLE_IRI_HTTP: &str = "http://schema.org/TechArticle";
/// <https://schema.org/TechArticle>
pub const TECH_ARTICLE_IRI_HTTPS: &str = "https://schema.org/TechArticle";
/// <https://schema.org/TechArticle>
pub const TECH_ARTICLE_LABEL: &str = "TechArticle";
pub struct TechArticleIri;
impl PartialEq<&str> for TechArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TECH_ARTICLE_IRI_HTTP || *other == TECH_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<TechArticleIri> for &str {
	fn eq(&self, other: &TechArticleIri) -> bool {
		*self == TECH_ARTICLE_IRI_HTTP || *self == TECH_ARTICLE_IRI_HTTPS
	}
}
pub struct TechArticleIriOrLabel;
impl PartialEq<&str> for TechArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TechArticleIri || *other == TECH_ARTICLE_LABEL
	}
}
impl PartialEq<TechArticleIriOrLabel> for &str {
	fn eq(&self, other: &TechArticleIriOrLabel) -> bool {
		*self == TechArticleIri || *self == TECH_ARTICLE_LABEL
	}
}
