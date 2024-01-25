/// <https://schema.org/acceptsReservations>
pub const ACCEPTS_RESERVATIONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/acceptsReservations";
/// <https://schema.org/acceptsReservations>
pub const ACCEPTS_RESERVATIONS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/acceptsReservations";
/// <https://schema.org/acceptsReservations>
pub const ACCEPTS_RESERVATIONS_PROPERTY_LABEL: &str = "acceptsReservations";
pub struct AcceptsReservationsPropertyIri;
impl PartialEq<&str> for AcceptsReservationsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCEPTS_RESERVATIONS_PROPERTY_IRI_HTTP
			|| *other == ACCEPTS_RESERVATIONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AcceptsReservationsPropertyIri> for &str {
	fn eq(&self, other: &AcceptsReservationsPropertyIri) -> bool {
		*self == ACCEPTS_RESERVATIONS_PROPERTY_IRI_HTTP
			|| *self == ACCEPTS_RESERVATIONS_PROPERTY_IRI_HTTPS
	}
}
pub struct AcceptsReservationsPropertyIriOrLabel;
impl PartialEq<&str> for AcceptsReservationsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AcceptsReservationsPropertyIri || *other == ACCEPTS_RESERVATIONS_PROPERTY_LABEL
	}
}
impl PartialEq<AcceptsReservationsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AcceptsReservationsPropertyIriOrLabel) -> bool {
		*self == AcceptsReservationsPropertyIri || *self == ACCEPTS_RESERVATIONS_PROPERTY_LABEL
	}
}
