/// <https://schema.org/MultiCenterTrial>
pub trait FindMultiCenterTrialIds {
	type IdType;
	/// <https://schema.org/MultiCenterTrial>
	fn find_multi_center_trial_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMultiCenterTrialIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_multi_center_trial_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MULTI_CENTER_TRIAL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MULTI_CENTER_TRIAL_IRI_HTTPS,
			})
		}
	}
}
