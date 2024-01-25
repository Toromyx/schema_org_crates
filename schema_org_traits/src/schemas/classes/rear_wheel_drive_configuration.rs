/// <https://schema.org/RearWheelDriveConfiguration>
pub trait FindRearWheelDriveConfigurationIds {
	type IdType;
	fn find_rear_wheel_drive_configuration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRearWheelDriveConfigurationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_rear_wheel_drive_configuration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::REAR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::REAR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
				}
			})
		}
	}
}
