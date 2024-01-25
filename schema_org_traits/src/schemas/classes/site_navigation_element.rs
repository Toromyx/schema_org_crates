/// <https://schema.org/SiteNavigationElement>
pub trait FindSiteNavigationElementIds {
	type IdType;
	fn find_site_navigation_element_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSiteNavigationElementIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_site_navigation_element_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SITE_NAVIGATION_ELEMENT_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::SITE_NAVIGATION_ELEMENT_IRI_HTTPS
				}
			})
		}
	}
}
