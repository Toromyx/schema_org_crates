/// <https://schema.org/GroupBoardingPolicy>
pub trait FindGroupBoardingPolicyIds {
	type IdType;
	fn find_group_boarding_policy_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGroupBoardingPolicyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_group_boarding_policy_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::GROUP_BOARDING_POLICY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::GROUP_BOARDING_POLICY_IRI_HTTPS,
			})
		}
	}
}
