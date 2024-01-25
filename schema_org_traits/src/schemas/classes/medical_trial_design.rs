/// <https://schema.org/MedicalTrialDesign>
pub trait FindMedicalTrialDesignIds {
	type IdType;
	fn find_medical_trial_design_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalTrialDesignIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_trial_design_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_TRIAL_DESIGN_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_TRIAL_DESIGN_IRI_HTTPS,
			})
		}
	}
}
