/// <https://schema.org/BodyMeasurementHeight>
pub trait FindBodyMeasurementHeightIds {
	type IdType;
	/// <https://schema.org/BodyMeasurementHeight>
	fn find_body_measurement_height_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBodyMeasurementHeightIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_body_measurement_height_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BODY_MEASUREMENT_HEIGHT_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::BODY_MEASUREMENT_HEIGHT_IRI_HTTPS
				}
			})
		}
	}
}
