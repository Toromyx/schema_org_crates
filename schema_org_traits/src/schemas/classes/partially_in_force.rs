/// <https://schema.org/PartiallyInForce>
pub trait FindPartiallyInForceIds {
	type IdType;
	/// <https://schema.org/PartiallyInForce>
	fn find_partially_in_force_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPartiallyInForceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_partially_in_force_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PARTIALLY_IN_FORCE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PARTIALLY_IN_FORCE_IRI_HTTPS,
			})
		}
	}
}
