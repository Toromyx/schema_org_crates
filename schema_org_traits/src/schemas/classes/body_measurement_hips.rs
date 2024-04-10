/// <https://schema.org/BodyMeasurementHips>
pub trait FindBodyMeasurementHipsIds {
	type IdType;
	/// <https://schema.org/BodyMeasurementHips>
	fn find_body_measurement_hips_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBodyMeasurementHipsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_body_measurement_hips_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BODY_MEASUREMENT_HIPS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BODY_MEASUREMENT_HIPS_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBodyMeasurementHipsIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_body_measurement_hips_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BODY_MEASUREMENT_HIPS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BODY_MEASUREMENT_HIPS_IRI_HTTPS,
			})
		}
	}
}
