/// <https://schema.org/OpinionNewsArticle>
pub trait FindOpinionNewsArticleIds {
	type IdType;
	/// <https://schema.org/OpinionNewsArticle>
	fn find_opinion_news_article_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOpinionNewsArticleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_opinion_news_article_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OPINION_NEWS_ARTICLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OPINION_NEWS_ARTICLE_IRI_HTTPS,
			})
		}
	}
}
