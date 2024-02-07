/// <https://schema.org/RightHandDriving>
pub trait FindRightHandDrivingIds {
	type IdType;
	/// <https://schema.org/RightHandDriving>
	fn find_right_hand_driving_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRightHandDrivingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_right_hand_driving_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RIGHT_HAND_DRIVING_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RIGHT_HAND_DRIVING_IRI_HTTPS,
			})
		}
	}
}
