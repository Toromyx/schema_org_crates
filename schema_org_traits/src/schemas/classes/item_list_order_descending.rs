/// <https://schema.org/ItemListOrderDescending>
pub trait FindItemListOrderDescendingIds {
	type IdType;
	fn find_item_list_order_descending_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindItemListOrderDescendingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_item_list_order_descending_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::ITEM_LIST_ORDER_DESCENDING_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::ITEM_LIST_ORDER_DESCENDING_IRI_HTTPS
				}
			})
		}
	}
}
