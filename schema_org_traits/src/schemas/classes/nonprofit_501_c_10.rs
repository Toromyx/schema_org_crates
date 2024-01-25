/// <https://schema.org/Nonprofit501c10>
pub trait FindNonprofit501C10Ids {
	type IdType;
	fn find_nonprofit_501_c_10_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNonprofit501C10Ids for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_nonprofit_501_c_10_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NONPROFIT_501_C_10_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NONPROFIT_501_C_10_IRI_HTTPS,
			})
		}
	}
}
