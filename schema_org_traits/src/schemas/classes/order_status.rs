/// <https://schema.org/OrderStatus>
pub trait FindOrderStatusIds {
	type IdType;
	/// <https://schema.org/OrderStatus>
	fn find_order_status_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOrderStatusIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_order_status_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ORDER_STATUS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ORDER_STATUS_IRI_HTTPS,
			})
		}
	}
}
