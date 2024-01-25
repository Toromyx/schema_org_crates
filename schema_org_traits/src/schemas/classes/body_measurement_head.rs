/// <https://schema.org/BodyMeasurementHead>
pub trait FindBodyMeasurementHeadIds {
	type IdType;
	fn find_body_measurement_head_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBodyMeasurementHeadIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_body_measurement_head_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BODY_MEASUREMENT_HEAD_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BODY_MEASUREMENT_HEAD_IRI_HTTPS,
			})
		}
	}
}
