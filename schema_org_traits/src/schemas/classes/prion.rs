/// <https://schema.org/Prion>
pub trait FindPrionIds {
	type IdType;
	/// <https://schema.org/Prion>
	fn find_prion_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPrionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_prion_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PRION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PRION_IRI_HTTPS,
			})
		}
	}
}
