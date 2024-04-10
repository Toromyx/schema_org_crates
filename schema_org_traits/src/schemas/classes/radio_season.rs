/// <https://schema.org/RadioSeason>
pub trait FindRadioSeasonIds {
	type IdType;
	/// <https://schema.org/RadioSeason>
	fn find_radio_season_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRadioSeasonIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_radio_season_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RADIO_SEASON_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RADIO_SEASON_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRadioSeasonIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_radio_season_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RADIO_SEASON_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RADIO_SEASON_IRI_HTTPS,
			})
		}
	}
}
