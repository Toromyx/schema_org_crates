/// <https://schema.org/MeetingRoom>
pub trait FindMeetingRoomIds {
	type IdType;
	/// <https://schema.org/MeetingRoom>
	fn find_meeting_room_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMeetingRoomIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_meeting_room_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEETING_ROOM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEETING_ROOM_IRI_HTTPS,
			})
		}
	}
}
