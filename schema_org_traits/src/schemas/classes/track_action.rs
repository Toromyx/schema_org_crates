/// <https://schema.org/TrackAction>
pub trait FindTrackActionIds {
	type IdType;
	/// <https://schema.org/TrackAction>
	fn find_track_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTrackActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_track_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TRACK_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TRACK_ACTION_IRI_HTTPS,
			})
		}
	}
}
