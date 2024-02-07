/// <https://schema.org/MulticellularParasite>
pub trait FindMulticellularParasiteIds {
	type IdType;
	/// <https://schema.org/MulticellularParasite>
	fn find_multicellular_parasite_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMulticellularParasiteIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_multicellular_parasite_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MULTICELLULAR_PARASITE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MULTICELLULAR_PARASITE_IRI_HTTPS,
			})
		}
	}
}
