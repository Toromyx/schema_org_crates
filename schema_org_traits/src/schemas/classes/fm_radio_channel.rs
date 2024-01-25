/// <https://schema.org/FMRadioChannel>
pub trait FindFmRadioChannelIds {
	type IdType;
	fn find_fm_radio_channel_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFmRadioChannelIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_fm_radio_channel_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FM_RADIO_CHANNEL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FM_RADIO_CHANNEL_IRI_HTTPS,
			})
		}
	}
}
