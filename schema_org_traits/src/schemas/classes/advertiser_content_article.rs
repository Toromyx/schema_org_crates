/// <https://schema.org/AdvertiserContentArticle>
pub trait FindAdvertiserContentArticleIds {
	type IdType;
	/// <https://schema.org/AdvertiserContentArticle>
	fn find_advertiser_content_article_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAdvertiserContentArticleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_advertiser_content_article_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::ADVERTISER_CONTENT_ARTICLE_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::ADVERTISER_CONTENT_ARTICLE_IRI_HTTPS
				}
			})
		}
	}
}
