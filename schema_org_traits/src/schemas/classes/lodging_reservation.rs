/// <https://schema.org/LodgingReservation>
pub trait FindLodgingReservationIds {
	type IdType;
	/// <https://schema.org/LodgingReservation>
	fn find_lodging_reservation_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLodgingReservationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_lodging_reservation_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LODGING_RESERVATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LODGING_RESERVATION_IRI_HTTPS,
			})
		}
	}
}
