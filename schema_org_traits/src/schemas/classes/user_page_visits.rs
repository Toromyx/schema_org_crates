/// <https://schema.org/UserPageVisits>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub trait FindUserPageVisitsIds {
	type IdType;
	#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
	fn find_user_page_visits_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUserPageVisitsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_user_page_visits_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::USER_PAGE_VISITS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::USER_PAGE_VISITS_IRI_HTTPS,
			})
		}
	}
}
