/// <https://schema.org/ChildCare>
pub trait FindChildCareIds {
	type IdType;
	/// <https://schema.org/ChildCare>
	fn find_child_care_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindChildCareIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_child_care_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CHILD_CARE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CHILD_CARE_IRI_HTTPS,
			})
		}
	}
}
