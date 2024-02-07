/// <https://schema.org/DoubleBlindedTrial>
pub trait FindDoubleBlindedTrialIds {
	type IdType;
	/// <https://schema.org/DoubleBlindedTrial>
	fn find_double_blinded_trial_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDoubleBlindedTrialIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_double_blinded_trial_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DOUBLE_BLINDED_TRIAL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DOUBLE_BLINDED_TRIAL_IRI_HTTPS,
			})
		}
	}
}
