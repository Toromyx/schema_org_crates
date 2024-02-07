/// <https://schema.org/NewsMediaOrganization>
pub trait FindNewsMediaOrganizationIds {
	type IdType;
	/// <https://schema.org/NewsMediaOrganization>
	fn find_news_media_organization_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNewsMediaOrganizationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_news_media_organization_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NEWS_MEDIA_ORGANIZATION_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::NEWS_MEDIA_ORGANIZATION_IRI_HTTPS
				}
			})
		}
	}
}
