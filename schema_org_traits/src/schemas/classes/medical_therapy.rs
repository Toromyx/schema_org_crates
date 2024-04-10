/// <https://schema.org/MedicalTherapy>
pub trait FindMedicalTherapyIds {
	type IdType;
	/// <https://schema.org/MedicalTherapy>
	fn find_medical_therapy_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalTherapyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_therapy_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_THERAPY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_THERAPY_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalTherapyIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_medical_therapy_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_THERAPY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_THERAPY_IRI_HTTPS,
			})
		}
	}
}
