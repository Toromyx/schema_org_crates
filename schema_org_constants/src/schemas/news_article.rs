/// <https://schema.org/NewsArticle>
pub const NEWS_ARTICLE_IRI_HTTP: &str = "http://schema.org/NewsArticle";
/// <https://schema.org/NewsArticle>
pub const NEWS_ARTICLE_IRI_HTTPS: &str = "https://schema.org/NewsArticle";
/// <https://schema.org/NewsArticle>
pub const NEWS_ARTICLE_LABEL: &str = "NewsArticle";
pub struct NewsArticleIri;
impl PartialEq<&str> for NewsArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEWS_ARTICLE_IRI_HTTP || *other == NEWS_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<NewsArticleIri> for &str {
	fn eq(&self, other: &NewsArticleIri) -> bool {
		*self == NEWS_ARTICLE_IRI_HTTP || *self == NEWS_ARTICLE_IRI_HTTPS
	}
}
pub struct NewsArticleIriOrLabel;
impl PartialEq<&str> for NewsArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NewsArticleIri || *other == NEWS_ARTICLE_LABEL
	}
}
impl PartialEq<NewsArticleIriOrLabel> for &str {
	fn eq(&self, other: &NewsArticleIriOrLabel) -> bool {
		*self == NewsArticleIri || *self == NEWS_ARTICLE_LABEL
	}
}
