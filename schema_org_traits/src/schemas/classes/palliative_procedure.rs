/// <https://schema.org/PalliativeProcedure>
pub trait FindPalliativeProcedureIds {
	type IdType;
	/// <https://schema.org/PalliativeProcedure>
	fn find_palliative_procedure_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPalliativeProcedureIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_palliative_procedure_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PALLIATIVE_PROCEDURE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PALLIATIVE_PROCEDURE_IRI_HTTPS,
			})
		}
	}
}
