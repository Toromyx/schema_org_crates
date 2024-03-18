/// <https://schema.org/ReservationHold>
pub trait FindReservationHoldIds {
	type IdType;
	/// <https://schema.org/ReservationHold>
	fn find_reservation_hold_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReservationHoldIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_reservation_hold_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RESERVATION_HOLD_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RESERVATION_HOLD_IRI_HTTPS,
			})
		}
	}
}