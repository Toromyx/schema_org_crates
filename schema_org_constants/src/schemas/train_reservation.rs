/// <https://schema.org/TrainReservation>
pub const TRAIN_RESERVATION_IRI_HTTP: &str = "http://schema.org/TrainReservation";
/// <https://schema.org/TrainReservation>
pub const TRAIN_RESERVATION_IRI_HTTPS: &str = "https://schema.org/TrainReservation";
/// <https://schema.org/TrainReservation>
pub const TRAIN_RESERVATION_LABEL: &str = "TrainReservation";
pub struct TrainReservationIri;
impl PartialEq<&str> for TrainReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAIN_RESERVATION_IRI_HTTP || *other == TRAIN_RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<TrainReservationIri> for &str {
	fn eq(&self, other: &TrainReservationIri) -> bool {
		*self == TRAIN_RESERVATION_IRI_HTTP || *self == TRAIN_RESERVATION_IRI_HTTPS
	}
}
pub struct TrainReservationIriOrLabel;
impl PartialEq<&str> for TrainReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrainReservationIri || *other == TRAIN_RESERVATION_LABEL
	}
}
impl PartialEq<TrainReservationIriOrLabel> for &str {
	fn eq(&self, other: &TrainReservationIriOrLabel) -> bool {
		*self == TrainReservationIri || *self == TRAIN_RESERVATION_LABEL
	}
}
