/// <https://schema.org/Casino>
pub trait FindCasinoIds {
	type IdType;
	/// <https://schema.org/Casino>
	fn find_casino_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCasinoIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_casino_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CASINO_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CASINO_IRI_HTTPS,
			})
		}
	}
}
