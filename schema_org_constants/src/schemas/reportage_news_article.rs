/// <https://schema.org/ReportageNewsArticle>
pub const REPORTAGE_NEWS_ARTICLE_IRI_HTTP: &str = "http://schema.org/ReportageNewsArticle";
/// <https://schema.org/ReportageNewsArticle>
pub const REPORTAGE_NEWS_ARTICLE_IRI_HTTPS: &str = "https://schema.org/ReportageNewsArticle";
/// <https://schema.org/ReportageNewsArticle>
pub const REPORTAGE_NEWS_ARTICLE_LABEL: &str = "ReportageNewsArticle";
pub struct ReportageNewsArticleIri;
impl PartialEq<&str> for ReportageNewsArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPORTAGE_NEWS_ARTICLE_IRI_HTTP || *other == REPORTAGE_NEWS_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<ReportageNewsArticleIri> for &str {
	fn eq(&self, other: &ReportageNewsArticleIri) -> bool {
		*self == REPORTAGE_NEWS_ARTICLE_IRI_HTTP || *self == REPORTAGE_NEWS_ARTICLE_IRI_HTTPS
	}
}
pub struct ReportageNewsArticleIriOrLabel;
impl PartialEq<&str> for ReportageNewsArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReportageNewsArticleIri || *other == REPORTAGE_NEWS_ARTICLE_LABEL
	}
}
impl PartialEq<ReportageNewsArticleIriOrLabel> for &str {
	fn eq(&self, other: &ReportageNewsArticleIriOrLabel) -> bool {
		*self == ReportageNewsArticleIri || *self == REPORTAGE_NEWS_ARTICLE_LABEL
	}
}
