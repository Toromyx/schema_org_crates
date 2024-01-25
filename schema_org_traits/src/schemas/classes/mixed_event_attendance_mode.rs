/// <https://schema.org/MixedEventAttendanceMode>
pub trait FindMixedEventAttendanceModeIds {
	type IdType;
	fn find_mixed_event_attendance_mode_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMixedEventAttendanceModeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_mixed_event_attendance_mode_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::MIXED_EVENT_ATTENDANCE_MODE_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::MIXED_EVENT_ATTENDANCE_MODE_IRI_HTTPS
				}
			})
		}
	}
}
