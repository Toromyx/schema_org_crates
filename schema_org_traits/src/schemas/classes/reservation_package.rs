/// <https://schema.org/ReservationPackage>
pub trait FindReservationPackageIds {
	type IdType;
	/// <https://schema.org/ReservationPackage>
	fn find_reservation_package_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReservationPackageIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_reservation_package_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RESERVATION_PACKAGE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RESERVATION_PACKAGE_IRI_HTTPS,
			})
		}
	}
}
