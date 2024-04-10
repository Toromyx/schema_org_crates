/// <https://schema.org/LeisureTimeActivity>
pub trait FindLeisureTimeActivityIds {
	type IdType;
	/// <https://schema.org/LeisureTimeActivity>
	fn find_leisure_time_activity_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLeisureTimeActivityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_leisure_time_activity_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LEISURE_TIME_ACTIVITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LEISURE_TIME_ACTIVITY_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLeisureTimeActivityIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_leisure_time_activity_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LEISURE_TIME_ACTIVITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LEISURE_TIME_ACTIVITY_IRI_HTTPS,
			})
		}
	}
}
