/// <https://schema.org/AnalysisNewsArticle>
pub const ANALYSIS_NEWS_ARTICLE_IRI_HTTP: &str = "http://schema.org/AnalysisNewsArticle";
/// <https://schema.org/AnalysisNewsArticle>
pub const ANALYSIS_NEWS_ARTICLE_IRI_HTTPS: &str = "https://schema.org/AnalysisNewsArticle";
/// <https://schema.org/AnalysisNewsArticle>
pub const ANALYSIS_NEWS_ARTICLE_LABEL: &str = "AnalysisNewsArticle";
pub struct AnalysisNewsArticleIri;
impl PartialEq<&str> for AnalysisNewsArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANALYSIS_NEWS_ARTICLE_IRI_HTTP || *other == ANALYSIS_NEWS_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<AnalysisNewsArticleIri> for &str {
	fn eq(&self, other: &AnalysisNewsArticleIri) -> bool {
		*self == ANALYSIS_NEWS_ARTICLE_IRI_HTTP || *self == ANALYSIS_NEWS_ARTICLE_IRI_HTTPS
	}
}
pub struct AnalysisNewsArticleIriOrLabel;
impl PartialEq<&str> for AnalysisNewsArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnalysisNewsArticleIri || *other == ANALYSIS_NEWS_ARTICLE_LABEL
	}
}
impl PartialEq<AnalysisNewsArticleIriOrLabel> for &str {
	fn eq(&self, other: &AnalysisNewsArticleIriOrLabel) -> bool {
		*self == AnalysisNewsArticleIri || *self == ANALYSIS_NEWS_ARTICLE_LABEL
	}
}
