/// <https://schema.org/eventAttendanceMode>
pub trait GetEventAttendanceModeProperty {
	type IdType;
	type PropertyType;
	fn get_event_attendance_mode_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetEventAttendanceModeProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_event_attendance_mode_property(
			&self,
			id: &Self::IdType,
		) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::EVENT_ATTENDANCE_MODE_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::EVENT_ATTENDANCE_MODE_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
