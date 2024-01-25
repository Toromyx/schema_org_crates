/// <https://schema.org/Article>
pub const ARTICLE_IRI_HTTP: &str = "http://schema.org/Article";
/// <https://schema.org/Article>
pub const ARTICLE_IRI_HTTPS: &str = "https://schema.org/Article";
/// <https://schema.org/Article>
pub const ARTICLE_LABEL: &str = "Article";
pub struct ArticleIri;
impl PartialEq<&str> for ArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARTICLE_IRI_HTTP || *other == ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<ArticleIri> for &str {
	fn eq(&self, other: &ArticleIri) -> bool {
		*self == ARTICLE_IRI_HTTP || *self == ARTICLE_IRI_HTTPS
	}
}
pub struct ArticleIriOrLabel;
impl PartialEq<&str> for ArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArticleIri || *other == ARTICLE_LABEL
	}
}
impl PartialEq<ArticleIriOrLabel> for &str {
	fn eq(&self, other: &ArticleIriOrLabel) -> bool {
		*self == ArticleIri || *self == ARTICLE_LABEL
	}
}
