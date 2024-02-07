/// <https://schema.org/ChemicalSubstance>
pub trait FindChemicalSubstanceIds {
	type IdType;
	/// <https://schema.org/ChemicalSubstance>
	fn find_chemical_substance_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindChemicalSubstanceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_chemical_substance_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CHEMICAL_SUBSTANCE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CHEMICAL_SUBSTANCE_IRI_HTTPS,
			})
		}
	}
}
