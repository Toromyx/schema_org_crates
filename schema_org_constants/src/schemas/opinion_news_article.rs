/// <https://schema.org/OpinionNewsArticle>
pub const OPINION_NEWS_ARTICLE_IRI_HTTP: &str = "http://schema.org/OpinionNewsArticle";
/// <https://schema.org/OpinionNewsArticle>
pub const OPINION_NEWS_ARTICLE_IRI_HTTPS: &str = "https://schema.org/OpinionNewsArticle";
/// <https://schema.org/OpinionNewsArticle>
pub const OPINION_NEWS_ARTICLE_LABEL: &str = "OpinionNewsArticle";
pub struct OpinionNewsArticleIri;
impl PartialEq<&str> for OpinionNewsArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPINION_NEWS_ARTICLE_IRI_HTTP || *other == OPINION_NEWS_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<OpinionNewsArticleIri> for &str {
	fn eq(&self, other: &OpinionNewsArticleIri) -> bool {
		*self == OPINION_NEWS_ARTICLE_IRI_HTTP || *self == OPINION_NEWS_ARTICLE_IRI_HTTPS
	}
}
pub struct OpinionNewsArticleIriOrLabel;
impl PartialEq<&str> for OpinionNewsArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OpinionNewsArticleIri || *other == OPINION_NEWS_ARTICLE_LABEL
	}
}
impl PartialEq<OpinionNewsArticleIriOrLabel> for &str {
	fn eq(&self, other: &OpinionNewsArticleIriOrLabel) -> bool {
		*self == OpinionNewsArticleIri || *self == OPINION_NEWS_ARTICLE_LABEL
	}
}
