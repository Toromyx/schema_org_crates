/// <https://schema.org/NoninvasiveProcedure>
pub trait FindNoninvasiveProcedureIds {
	type IdType;
	/// <https://schema.org/NoninvasiveProcedure>
	fn find_noninvasive_procedure_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNoninvasiveProcedureIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_noninvasive_procedure_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NONINVASIVE_PROCEDURE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NONINVASIVE_PROCEDURE_IRI_HTTPS,
			})
		}
	}
}
