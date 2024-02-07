/// <https://schema.org/DriveWheelConfigurationValue>
pub trait FindDriveWheelConfigurationValueIds {
	type IdType;
	/// <https://schema.org/DriveWheelConfigurationValue>
	fn find_drive_wheel_configuration_value_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDriveWheelConfigurationValueIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_drive_wheel_configuration_value_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::DRIVE_WHEEL_CONFIGURATION_VALUE_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::DRIVE_WHEEL_CONFIGURATION_VALUE_IRI_HTTPS
				}
			})
		}
	}
}
