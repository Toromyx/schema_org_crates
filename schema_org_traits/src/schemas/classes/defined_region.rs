/// <https://schema.org/DefinedRegion>
pub trait FindDefinedRegionIds {
	type IdType;
	/// <https://schema.org/DefinedRegion>
	fn find_defined_region_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDefinedRegionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_defined_region_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DEFINED_REGION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DEFINED_REGION_IRI_HTTPS,
			})
		}
	}
}
