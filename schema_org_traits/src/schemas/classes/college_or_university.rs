/// <https://schema.org/CollegeOrUniversity>
pub trait FindCollegeOrUniversityIds {
	type IdType;
	/// <https://schema.org/CollegeOrUniversity>
	fn find_college_or_university_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCollegeOrUniversityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_college_or_university_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::COLLEGE_OR_UNIVERSITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::COLLEGE_OR_UNIVERSITY_IRI_HTTPS,
			})
		}
	}
}
