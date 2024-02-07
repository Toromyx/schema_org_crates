/// <https://schema.org/BodyMeasurementWaist>
pub trait FindBodyMeasurementWaistIds {
	type IdType;
	/// <https://schema.org/BodyMeasurementWaist>
	fn find_body_measurement_waist_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBodyMeasurementWaistIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_body_measurement_waist_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BODY_MEASUREMENT_WAIST_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BODY_MEASUREMENT_WAIST_IRI_HTTPS,
			})
		}
	}
}
