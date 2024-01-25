/// <https://schema.org/subReservation>
pub const SUB_RESERVATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/subReservation";
/// <https://schema.org/subReservation>
pub const SUB_RESERVATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subReservation";
/// <https://schema.org/subReservation>
pub const SUB_RESERVATION_PROPERTY_LABEL: &str = "subReservation";
pub struct SubReservationPropertyIri;
impl PartialEq<&str> for SubReservationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUB_RESERVATION_PROPERTY_IRI_HTTP || *other == SUB_RESERVATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubReservationPropertyIri> for &str {
	fn eq(&self, other: &SubReservationPropertyIri) -> bool {
		*self == SUB_RESERVATION_PROPERTY_IRI_HTTP || *self == SUB_RESERVATION_PROPERTY_IRI_HTTPS
	}
}
pub struct SubReservationPropertyIriOrLabel;
impl PartialEq<&str> for SubReservationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubReservationPropertyIri || *other == SUB_RESERVATION_PROPERTY_LABEL
	}
}
impl PartialEq<SubReservationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubReservationPropertyIriOrLabel) -> bool {
		*self == SubReservationPropertyIri || *self == SUB_RESERVATION_PROPERTY_LABEL
	}
}
