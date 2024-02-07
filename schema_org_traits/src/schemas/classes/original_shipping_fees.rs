/// <https://schema.org/OriginalShippingFees>
pub trait FindOriginalShippingFeesIds {
	type IdType;
	/// <https://schema.org/OriginalShippingFees>
	fn find_original_shipping_fees_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOriginalShippingFeesIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_original_shipping_fees_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ORIGINAL_SHIPPING_FEES_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ORIGINAL_SHIPPING_FEES_IRI_HTTPS,
			})
		}
	}
}
