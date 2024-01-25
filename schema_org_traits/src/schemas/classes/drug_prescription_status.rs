/// <https://schema.org/DrugPrescriptionStatus>
pub trait FindDrugPrescriptionStatusIds {
	type IdType;
	fn find_drug_prescription_status_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDrugPrescriptionStatusIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_drug_prescription_status_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DRUG_PRESCRIPTION_STATUS_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::DRUG_PRESCRIPTION_STATUS_IRI_HTTPS
				}
			})
		}
	}
}
