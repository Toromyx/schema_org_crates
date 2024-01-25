/// <https://schema.org/TechArticle>
pub trait FindTechArticleIds {
	type IdType;
	fn find_tech_article_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTechArticleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_tech_article_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TECH_ARTICLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TECH_ARTICLE_IRI_HTTPS,
			})
		}
	}
}
