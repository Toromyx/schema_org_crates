/// <https://schema.org/MSRP>
pub trait FindMsrpIds {
	type IdType;
	fn find_msrp_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMsrpIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_msrp_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MSRP_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MSRP_IRI_HTTPS,
			})
		}
	}
}
