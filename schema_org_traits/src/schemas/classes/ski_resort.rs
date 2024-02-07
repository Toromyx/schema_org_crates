/// <https://schema.org/SkiResort>
pub trait FindSkiResortIds {
	type IdType;
	/// <https://schema.org/SkiResort>
	fn find_ski_resort_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSkiResortIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_ski_resort_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SKI_RESORT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SKI_RESORT_IRI_HTTPS,
			})
		}
	}
}
