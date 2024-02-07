/// <https://schema.org/TouristInformationCenter>
pub trait FindTouristInformationCenterIds {
	type IdType;
	/// <https://schema.org/TouristInformationCenter>
	fn find_tourist_information_center_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTouristInformationCenterIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_tourist_information_center_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::TOURIST_INFORMATION_CENTER_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::TOURIST_INFORMATION_CENTER_IRI_HTTPS
				}
			})
		}
	}
}
