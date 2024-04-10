/// <https://schema.org/ItemPage>
pub trait FindItemPageIds {
	type IdType;
	/// <https://schema.org/ItemPage>
	fn find_item_page_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindItemPageIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_item_page_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ITEM_PAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ITEM_PAGE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindItemPageIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_item_page_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ITEM_PAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ITEM_PAGE_IRI_HTTPS,
			})
		}
	}
}
