/// <https://schema.org/TravelAgency>
pub trait FindTravelAgencyIds {
	type IdType;
	/// <https://schema.org/TravelAgency>
	fn find_travel_agency_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTravelAgencyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_travel_agency_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TRAVEL_AGENCY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TRAVEL_AGENCY_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTravelAgencyIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_travel_agency_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TRAVEL_AGENCY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TRAVEL_AGENCY_IRI_HTTPS,
			})
		}
	}
}
