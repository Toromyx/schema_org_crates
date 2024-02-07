/// <https://schema.org/FrontWheelDriveConfiguration>
pub trait FindFrontWheelDriveConfigurationIds {
	type IdType;
	/// <https://schema.org/FrontWheelDriveConfiguration>
	fn find_front_wheel_drive_configuration_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFrontWheelDriveConfigurationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_front_wheel_drive_configuration_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::FRONT_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::FRONT_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
				}
			})
		}
	}
}
