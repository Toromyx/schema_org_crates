/// <https://schema.org/EnergyStarCertified>
pub trait FindEnergyStarCertifiedIds {
	type IdType;
	/// <https://schema.org/EnergyStarCertified>
	fn find_energy_star_certified_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEnergyStarCertifiedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_energy_star_certified_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ENERGY_STAR_CERTIFIED_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ENERGY_STAR_CERTIFIED_IRI_HTTPS,
			})
		}
	}
}
