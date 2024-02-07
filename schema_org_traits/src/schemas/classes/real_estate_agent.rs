/// <https://schema.org/RealEstateAgent>
pub trait FindRealEstateAgentIds {
	type IdType;
	/// <https://schema.org/RealEstateAgent>
	fn find_real_estate_agent_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRealEstateAgentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_real_estate_agent_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::REAL_ESTATE_AGENT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::REAL_ESTATE_AGENT_IRI_HTTPS,
			})
		}
	}
}
