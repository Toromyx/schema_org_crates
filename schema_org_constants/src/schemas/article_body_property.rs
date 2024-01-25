/// <https://schema.org/articleBody>
pub const ARTICLE_BODY_PROPERTY_IRI_HTTP: &str = "http://schema.org/articleBody";
/// <https://schema.org/articleBody>
pub const ARTICLE_BODY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/articleBody";
/// <https://schema.org/articleBody>
pub const ARTICLE_BODY_PROPERTY_LABEL: &str = "articleBody";
pub struct ArticleBodyPropertyIri;
impl PartialEq<&str> for ArticleBodyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARTICLE_BODY_PROPERTY_IRI_HTTP || *other == ARTICLE_BODY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArticleBodyPropertyIri> for &str {
	fn eq(&self, other: &ArticleBodyPropertyIri) -> bool {
		*self == ARTICLE_BODY_PROPERTY_IRI_HTTP || *self == ARTICLE_BODY_PROPERTY_IRI_HTTPS
	}
}
pub struct ArticleBodyPropertyIriOrLabel;
impl PartialEq<&str> for ArticleBodyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArticleBodyPropertyIri || *other == ARTICLE_BODY_PROPERTY_LABEL
	}
}
impl PartialEq<ArticleBodyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArticleBodyPropertyIriOrLabel) -> bool {
		*self == ArticleBodyPropertyIri || *self == ARTICLE_BODY_PROPERTY_LABEL
	}
}
