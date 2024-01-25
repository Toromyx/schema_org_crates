/// <https://schema.org/associatedArticle>
pub const ASSOCIATED_ARTICLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/associatedArticle";
/// <https://schema.org/associatedArticle>
pub const ASSOCIATED_ARTICLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/associatedArticle";
/// <https://schema.org/associatedArticle>
pub const ASSOCIATED_ARTICLE_PROPERTY_LABEL: &str = "associatedArticle";
pub struct AssociatedArticlePropertyIri;
impl PartialEq<&str> for AssociatedArticlePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSOCIATED_ARTICLE_PROPERTY_IRI_HTTP
			|| *other == ASSOCIATED_ARTICLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssociatedArticlePropertyIri> for &str {
	fn eq(&self, other: &AssociatedArticlePropertyIri) -> bool {
		*self == ASSOCIATED_ARTICLE_PROPERTY_IRI_HTTP
			|| *self == ASSOCIATED_ARTICLE_PROPERTY_IRI_HTTPS
	}
}
pub struct AssociatedArticlePropertyIriOrLabel;
impl PartialEq<&str> for AssociatedArticlePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssociatedArticlePropertyIri || *other == ASSOCIATED_ARTICLE_PROPERTY_LABEL
	}
}
impl PartialEq<AssociatedArticlePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssociatedArticlePropertyIriOrLabel) -> bool {
		*self == AssociatedArticlePropertyIri || *self == ASSOCIATED_ARTICLE_PROPERTY_LABEL
	}
}
