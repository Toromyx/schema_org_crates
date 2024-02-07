/// <https://schema.org/JobPosting>
pub trait FindJobPostingIds {
	type IdType;
	/// <https://schema.org/JobPosting>
	fn find_job_posting_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindJobPostingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_job_posting_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::JOB_POSTING_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::JOB_POSTING_IRI_HTTPS,
			})
		}
	}
}
