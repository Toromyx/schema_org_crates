/// <https://schema.org/WearableMeasurementTypeEnumeration>
pub trait FindWearableMeasurementTypeEnumerationIds {
	type IdType;
	/// <https://schema.org/WearableMeasurementTypeEnumeration>
	fn find_wearable_measurement_type_enumeration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableMeasurementTypeEnumerationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_measurement_type_enumeration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS
				}
			})
		}
	}
}
