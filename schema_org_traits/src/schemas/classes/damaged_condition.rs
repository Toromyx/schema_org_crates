/// <https://schema.org/DamagedCondition>
pub trait FindDamagedConditionIds {
	type IdType;
	/// <https://schema.org/DamagedCondition>
	fn find_damaged_condition_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDamagedConditionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_damaged_condition_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DAMAGED_CONDITION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DAMAGED_CONDITION_IRI_HTTPS,
			})
		}
	}
}
