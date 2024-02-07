/// <https://schema.org/SingleBlindedTrial>
pub trait FindSingleBlindedTrialIds {
	type IdType;
	/// <https://schema.org/SingleBlindedTrial>
	fn find_single_blinded_trial_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSingleBlindedTrialIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_single_blinded_trial_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SINGLE_BLINDED_TRIAL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SINGLE_BLINDED_TRIAL_IRI_HTTPS,
			})
		}
	}
}
