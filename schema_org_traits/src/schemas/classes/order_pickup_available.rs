/// <https://schema.org/OrderPickupAvailable>
pub trait FindOrderPickupAvailableIds {
	type IdType;
	fn find_order_pickup_available_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOrderPickupAvailableIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_order_pickup_available_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ORDER_PICKUP_AVAILABLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ORDER_PICKUP_AVAILABLE_IRI_HTTPS,
			})
		}
	}
}
