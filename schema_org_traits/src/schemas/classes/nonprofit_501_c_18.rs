/// <https://schema.org/Nonprofit501c18>
pub trait FindNonprofit501C18Ids {
	type IdType;
	fn find_nonprofit_501_c_18_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNonprofit501C18Ids for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_nonprofit_501_c_18_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NONPROFIT_501_C_18_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NONPROFIT_501_C_18_IRI_HTTPS,
			})
		}
	}
}
