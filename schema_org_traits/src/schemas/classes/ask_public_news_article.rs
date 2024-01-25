/// <https://schema.org/AskPublicNewsArticle>
pub trait FindAskPublicNewsArticleIds {
	type IdType;
	fn find_ask_public_news_article_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAskPublicNewsArticleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_ask_public_news_article_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ASK_PUBLIC_NEWS_ARTICLE_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::ASK_PUBLIC_NEWS_ARTICLE_IRI_HTTPS
				}
			})
		}
	}
}
