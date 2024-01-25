/// <https://schema.org/TrainTrip>
pub const TRAIN_TRIP_IRI_HTTP: &str = "http://schema.org/TrainTrip";
/// <https://schema.org/TrainTrip>
pub const TRAIN_TRIP_IRI_HTTPS: &str = "https://schema.org/TrainTrip";
/// <https://schema.org/TrainTrip>
pub const TRAIN_TRIP_LABEL: &str = "TrainTrip";
pub struct TrainTripIri;
impl PartialEq<&str> for TrainTripIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAIN_TRIP_IRI_HTTP || *other == TRAIN_TRIP_IRI_HTTPS
	}
}
impl PartialEq<TrainTripIri> for &str {
	fn eq(&self, other: &TrainTripIri) -> bool {
		*self == TRAIN_TRIP_IRI_HTTP || *self == TRAIN_TRIP_IRI_HTTPS
	}
}
pub struct TrainTripIriOrLabel;
impl PartialEq<&str> for TrainTripIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrainTripIri || *other == TRAIN_TRIP_LABEL
	}
}
impl PartialEq<TrainTripIriOrLabel> for &str {
	fn eq(&self, other: &TrainTripIriOrLabel) -> bool {
		*self == TrainTripIri || *self == TRAIN_TRIP_LABEL
	}
}
