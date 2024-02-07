/// <https://schema.org/CT>
pub trait FindCtIds {
	type IdType;
	/// <https://schema.org/CT>
	fn find_ct_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCtIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_ct_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CT_IRI_HTTPS,
			})
		}
	}
}
