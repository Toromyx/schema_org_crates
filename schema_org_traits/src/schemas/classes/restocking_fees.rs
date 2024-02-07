/// <https://schema.org/RestockingFees>
pub trait FindRestockingFeesIds {
	type IdType;
	/// <https://schema.org/RestockingFees>
	fn find_restocking_fees_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRestockingFeesIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_restocking_fees_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RESTOCKING_FEES_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RESTOCKING_FEES_IRI_HTTPS,
			})
		}
	}
}
