/// <https://schema.org/MedicalIntangible>
pub trait FindMedicalIntangibleIds {
	type IdType;
	/// <https://schema.org/MedicalIntangible>
	fn find_medical_intangible_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalIntangibleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_intangible_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_INTANGIBLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_INTANGIBLE_IRI_HTTPS,
			})
		}
	}
}
