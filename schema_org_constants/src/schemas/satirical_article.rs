/// <https://schema.org/SatiricalArticle>
pub const SATIRICAL_ARTICLE_IRI_HTTP: &str = "http://schema.org/SatiricalArticle";
/// <https://schema.org/SatiricalArticle>
pub const SATIRICAL_ARTICLE_IRI_HTTPS: &str = "https://schema.org/SatiricalArticle";
/// <https://schema.org/SatiricalArticle>
pub const SATIRICAL_ARTICLE_LABEL: &str = "SatiricalArticle";
pub struct SatiricalArticleIri;
impl PartialEq<&str> for SatiricalArticleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SATIRICAL_ARTICLE_IRI_HTTP || *other == SATIRICAL_ARTICLE_IRI_HTTPS
	}
}
impl PartialEq<SatiricalArticleIri> for &str {
	fn eq(&self, other: &SatiricalArticleIri) -> bool {
		*self == SATIRICAL_ARTICLE_IRI_HTTP || *self == SATIRICAL_ARTICLE_IRI_HTTPS
	}
}
pub struct SatiricalArticleIriOrLabel;
impl PartialEq<&str> for SatiricalArticleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SatiricalArticleIri || *other == SATIRICAL_ARTICLE_LABEL
	}
}
impl PartialEq<SatiricalArticleIriOrLabel> for &str {
	fn eq(&self, other: &SatiricalArticleIriOrLabel) -> bool {
		*self == SatiricalArticleIri || *self == SATIRICAL_ARTICLE_LABEL
	}
}
