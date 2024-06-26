/// <https://schema.org/TelevisionStation>
pub trait FindTelevisionStationIds {
	type IdType;
	/// <https://schema.org/TelevisionStation>
	fn find_television_station_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTelevisionStationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_television_station_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TELEVISION_STATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TELEVISION_STATION_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTelevisionStationIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_television_station_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TELEVISION_STATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TELEVISION_STATION_IRI_HTTPS,
			})
		}
	}
}
