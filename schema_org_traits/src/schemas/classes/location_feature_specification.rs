/// <https://schema.org/LocationFeatureSpecification>
pub trait FindLocationFeatureSpecificationIds {
	type IdType;
	fn find_location_feature_specification_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLocationFeatureSpecificationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_location_feature_specification_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::LOCATION_FEATURE_SPECIFICATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::LOCATION_FEATURE_SPECIFICATION_IRI_HTTPS
				}
			})
		}
	}
}
