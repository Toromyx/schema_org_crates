/// <https://schema.org/HowToSupply>
pub trait FindHowToSupplyIds {
	type IdType;
	/// <https://schema.org/HowToSupply>
	fn find_how_to_supply_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHowToSupplyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_how_to_supply_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HOW_TO_SUPPLY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HOW_TO_SUPPLY_IRI_HTTPS,
			})
		}
	}
}
