/// <https://schema.org/PsychologicalTreatment>
pub trait FindPsychologicalTreatmentIds {
	type IdType;
	/// <https://schema.org/PsychologicalTreatment>
	fn find_psychological_treatment_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPsychologicalTreatmentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_psychological_treatment_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PSYCHOLOGICAL_TREATMENT_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::PSYCHOLOGICAL_TREATMENT_IRI_HTTPS
				}
			})
		}
	}
}
