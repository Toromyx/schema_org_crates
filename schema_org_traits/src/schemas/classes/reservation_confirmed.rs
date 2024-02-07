/// <https://schema.org/ReservationConfirmed>
pub trait FindReservationConfirmedIds {
	type IdType;
	/// <https://schema.org/ReservationConfirmed>
	fn find_reservation_confirmed_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReservationConfirmedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_reservation_confirmed_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RESERVATION_CONFIRMED_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RESERVATION_CONFIRMED_IRI_HTTPS,
			})
		}
	}
}
