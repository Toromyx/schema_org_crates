/// <https://schema.org/BackgroundNewsArticle>
pub trait FindBackgroundNewsArticleIds {
	type IdType;
	fn find_background_news_article_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBackgroundNewsArticleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_background_news_article_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BACKGROUND_NEWS_ARTICLE_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::BACKGROUND_NEWS_ARTICLE_IRI_HTTPS
				}
			})
		}
	}
}
