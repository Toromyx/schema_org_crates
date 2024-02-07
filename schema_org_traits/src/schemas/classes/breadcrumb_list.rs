/// <https://schema.org/BreadcrumbList>
pub trait FindBreadcrumbListIds {
	type IdType;
	/// <https://schema.org/BreadcrumbList>
	fn find_breadcrumb_list_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBreadcrumbListIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_breadcrumb_list_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BREADCRUMB_LIST_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BREADCRUMB_LIST_IRI_HTTPS,
			})
		}
	}
}
