/// <https://schema.org/ReviewNewsArticle>
pub const REVIEW_NEWS_ARTICLE_IRI_HTTP: &str = "http://schema.org/ReviewNewsArticle";
/// <https://schema.org/ReviewNewsArticle>
pub const REVIEW_NEWS_ARTICLE_IRI_HTTPS: &str = "https://schema.org/ReviewNewsArticle";
/// <https://schema.org/ReviewNewsArticle>
pub const REVIEW_NEWS_ARTICLE_LABEL: &str = "ReviewNewsArticle";
pub struct ReviewNewsArticleIri;
impl PartialEq<&str> for ReviewNewsArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEW_NEWS_ARTICLE_IRI_HTTP || *other == REVIEW_NEWS_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<ReviewNewsArticleIri> for &str {
	fn eq(&self, other: &ReviewNewsArticleIri) -> bool {
		*self == REVIEW_NEWS_ARTICLE_IRI_HTTP || *self == REVIEW_NEWS_ARTICLE_IRI_HTTPS
	}
}
pub struct ReviewNewsArticleIriOrLabel;
impl PartialEq<&str> for ReviewNewsArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewNewsArticleIri || *other == REVIEW_NEWS_ARTICLE_LABEL
	}
}
impl PartialEq<ReviewNewsArticleIriOrLabel> for &str {
	fn eq(&self, other: &ReviewNewsArticleIriOrLabel) -> bool {
		*self == ReviewNewsArticleIri || *self == REVIEW_NEWS_ARTICLE_LABEL
	}
}
