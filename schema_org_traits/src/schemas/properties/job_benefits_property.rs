/// <https://schema.org/jobBenefits>
pub trait GetJobBenefitsProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/jobBenefits>
	fn get_job_benefits_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetJobBenefitsProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_job_benefits_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::JOB_BENEFITS_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::JOB_BENEFITS_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetJobBenefitsProperty for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		type PropertyType = rdf_types_0_22::Object;
		fn get_job_benefits_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::JOB_BENEFITS_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::JOB_BENEFITS_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
