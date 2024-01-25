/// <https://schema.org/AskPublicNewsArticle>
pub const ASK_PUBLIC_NEWS_ARTICLE_IRI_HTTP: &str = "http://schema.org/AskPublicNewsArticle";
/// <https://schema.org/AskPublicNewsArticle>
pub const ASK_PUBLIC_NEWS_ARTICLE_IRI_HTTPS: &str = "https://schema.org/AskPublicNewsArticle";
/// <https://schema.org/AskPublicNewsArticle>
pub const ASK_PUBLIC_NEWS_ARTICLE_LABEL: &str = "AskPublicNewsArticle";
pub struct AskPublicNewsArticleIri;
impl PartialEq<&str> for AskPublicNewsArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASK_PUBLIC_NEWS_ARTICLE_IRI_HTTP || *other == ASK_PUBLIC_NEWS_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<AskPublicNewsArticleIri> for &str {
	fn eq(&self, other: &AskPublicNewsArticleIri) -> bool {
		*self == ASK_PUBLIC_NEWS_ARTICLE_IRI_HTTP || *self == ASK_PUBLIC_NEWS_ARTICLE_IRI_HTTPS
	}
}
pub struct AskPublicNewsArticleIriOrLabel;
impl PartialEq<&str> for AskPublicNewsArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AskPublicNewsArticleIri || *other == ASK_PUBLIC_NEWS_ARTICLE_LABEL
	}
}
impl PartialEq<AskPublicNewsArticleIriOrLabel> for &str {
	fn eq(&self, other: &AskPublicNewsArticleIriOrLabel) -> bool {
		*self == AskPublicNewsArticleIri || *self == ASK_PUBLIC_NEWS_ARTICLE_LABEL
	}
}
