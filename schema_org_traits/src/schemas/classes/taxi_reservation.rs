/// <https://schema.org/TaxiReservation>
pub trait FindTaxiReservationIds {
	type IdType;
	/// <https://schema.org/TaxiReservation>
	fn find_taxi_reservation_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindTaxiReservationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_taxi_reservation_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::TAXI_RESERVATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::TAXI_RESERVATION_IRI_HTTPS,
			})
		}
	}
}
