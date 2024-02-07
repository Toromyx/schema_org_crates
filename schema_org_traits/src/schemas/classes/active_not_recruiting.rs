/// <https://schema.org/ActiveNotRecruiting>
pub trait FindActiveNotRecruitingIds {
	type IdType;
	/// <https://schema.org/ActiveNotRecruiting>
	fn find_active_not_recruiting_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindActiveNotRecruitingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_active_not_recruiting_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ACTIVE_NOT_RECRUITING_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ACTIVE_NOT_RECRUITING_IRI_HTTPS,
			})
		}
	}
}
