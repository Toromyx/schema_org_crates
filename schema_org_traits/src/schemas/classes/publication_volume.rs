/// <https://schema.org/PublicationVolume>
pub trait FindPublicationVolumeIds {
	type IdType;
	/// <https://schema.org/PublicationVolume>
	fn find_publication_volume_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPublicationVolumeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_publication_volume_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PUBLICATION_VOLUME_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PUBLICATION_VOLUME_IRI_HTTPS,
			})
		}
	}
}
