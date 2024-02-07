/// <https://schema.org/WearableMeasurementOutsideLeg>
pub trait FindWearableMeasurementOutsideLegIds {
	type IdType;
	/// <https://schema.org/WearableMeasurementOutsideLeg>
	fn find_wearable_measurement_outside_leg_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableMeasurementOutsideLegIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_measurement_outside_leg_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_MEASUREMENT_OUTSIDE_LEG_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_MEASUREMENT_OUTSIDE_LEG_IRI_HTTPS
				}
			})
		}
	}
}
