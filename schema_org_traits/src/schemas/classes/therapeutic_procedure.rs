/// <https://schema.org/TherapeuticProcedure>
pub trait FindTherapeuticProcedureIds {
	type IdType;
	fn find_therapeutic_procedure_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTherapeuticProcedureIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_therapeutic_procedure_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::THERAPEUTIC_PROCEDURE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::THERAPEUTIC_PROCEDURE_IRI_HTTPS,
			})
		}
	}
}
