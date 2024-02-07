/// <https://schema.org/NotInForce>
pub trait FindNotInForceIds {
	type IdType;
	/// <https://schema.org/NotInForce>
	fn find_not_in_force_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNotInForceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_not_in_force_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NOT_IN_FORCE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NOT_IN_FORCE_IRI_HTTPS,
			})
		}
	}
}
