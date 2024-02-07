/// <https://schema.org/FullRefund>
pub trait FindFullRefundIds {
	type IdType;
	/// <https://schema.org/FullRefund>
	fn find_full_refund_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFullRefundIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_full_refund_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FULL_REFUND_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FULL_REFUND_IRI_HTTPS,
			})
		}
	}
}
