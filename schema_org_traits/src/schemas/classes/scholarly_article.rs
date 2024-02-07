/// <https://schema.org/ScholarlyArticle>
pub trait FindScholarlyArticleIds {
	type IdType;
	/// <https://schema.org/ScholarlyArticle>
	fn find_scholarly_article_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindScholarlyArticleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_scholarly_article_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SCHOLARLY_ARTICLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SCHOLARLY_ARTICLE_IRI_HTTPS,
			})
		}
	}
}
