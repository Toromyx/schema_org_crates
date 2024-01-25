/// <https://schema.org/BackgroundNewsArticle>
pub const BACKGROUND_NEWS_ARTICLE_IRI_HTTP: &str = "http://schema.org/BackgroundNewsArticle";
/// <https://schema.org/BackgroundNewsArticle>
pub const BACKGROUND_NEWS_ARTICLE_IRI_HTTPS: &str = "https://schema.org/BackgroundNewsArticle";
/// <https://schema.org/BackgroundNewsArticle>
pub const BACKGROUND_NEWS_ARTICLE_LABEL: &str = "BackgroundNewsArticle";
pub struct BackgroundNewsArticleIri;
impl PartialEq<&str> for BackgroundNewsArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BACKGROUND_NEWS_ARTICLE_IRI_HTTP || *other == BACKGROUND_NEWS_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<BackgroundNewsArticleIri> for &str {
	fn eq(&self, other: &BackgroundNewsArticleIri) -> bool {
		*self == BACKGROUND_NEWS_ARTICLE_IRI_HTTP || *self == BACKGROUND_NEWS_ARTICLE_IRI_HTTPS
	}
}
pub struct BackgroundNewsArticleIriOrLabel;
impl PartialEq<&str> for BackgroundNewsArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BackgroundNewsArticleIri || *other == BACKGROUND_NEWS_ARTICLE_LABEL
	}
}
impl PartialEq<BackgroundNewsArticleIriOrLabel> for &str {
	fn eq(&self, other: &BackgroundNewsArticleIriOrLabel) -> bool {
		*self == BackgroundNewsArticleIri || *self == BACKGROUND_NEWS_ARTICLE_LABEL
	}
}
