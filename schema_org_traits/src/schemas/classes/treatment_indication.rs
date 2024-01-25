/// <https://schema.org/TreatmentIndication>
pub trait FindTreatmentIndicationIds {
	type IdType;
	fn find_treatment_indication_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTreatmentIndicationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_treatment_indication_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TREATMENT_INDICATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TREATMENT_INDICATION_IRI_HTTPS,
			})
		}
	}
}
