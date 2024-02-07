/// <https://schema.org/Locksmith>
pub trait FindLocksmithIds {
	type IdType;
	/// <https://schema.org/Locksmith>
	fn find_locksmith_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLocksmithIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_locksmith_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LOCKSMITH_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LOCKSMITH_IRI_HTTPS,
			})
		}
	}
}
