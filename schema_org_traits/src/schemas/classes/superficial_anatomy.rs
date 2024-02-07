/// <https://schema.org/SuperficialAnatomy>
pub trait FindSuperficialAnatomyIds {
	type IdType;
	/// <https://schema.org/SuperficialAnatomy>
	fn find_superficial_anatomy_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSuperficialAnatomyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_superficial_anatomy_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SUPERFICIAL_ANATOMY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SUPERFICIAL_ANATOMY_IRI_HTTPS,
			})
		}
	}
}
