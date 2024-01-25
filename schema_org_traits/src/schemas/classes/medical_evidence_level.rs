/// <https://schema.org/MedicalEvidenceLevel>
pub trait FindMedicalEvidenceLevelIds {
	type IdType;
	fn find_medical_evidence_level_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalEvidenceLevelIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_evidence_level_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_EVIDENCE_LEVEL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_EVIDENCE_LEVEL_IRI_HTTPS,
			})
		}
	}
}
