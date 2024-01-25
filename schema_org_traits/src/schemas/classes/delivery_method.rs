/// <https://schema.org/DeliveryMethod>
pub trait FindDeliveryMethodIds {
	type IdType;
	fn find_delivery_method_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDeliveryMethodIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_delivery_method_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DELIVERY_METHOD_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DELIVERY_METHOD_IRI_HTTPS,
			})
		}
	}
}
