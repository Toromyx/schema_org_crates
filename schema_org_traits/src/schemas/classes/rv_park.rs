/// <https://schema.org/RVPark>
pub trait FindRvParkIds {
	type IdType;
	/// <https://schema.org/RVPark>
	fn find_rv_park_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRvParkIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_rv_park_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RV_PARK_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RV_PARK_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRvParkIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_rv_park_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RV_PARK_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RV_PARK_IRI_HTTPS,
			})
		}
	}
}
