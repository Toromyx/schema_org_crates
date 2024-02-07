/// <https://schema.org/MedicalProcedureType>
pub trait FindMedicalProcedureTypeIds {
	type IdType;
	/// <https://schema.org/MedicalProcedureType>
	fn find_medical_procedure_type_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalProcedureTypeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_procedure_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_PROCEDURE_TYPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_PROCEDURE_TYPE_IRI_HTTPS,
			})
		}
	}
}
