/// <https://schema.org/TrainStation>
pub const TRAIN_STATION_IRI_HTTP: &str = "http://schema.org/TrainStation";
/// <https://schema.org/TrainStation>
pub const TRAIN_STATION_IRI_HTTPS: &str = "https://schema.org/TrainStation";
/// <https://schema.org/TrainStation>
pub const TRAIN_STATION_LABEL: &str = "TrainStation";
pub struct TrainStationIri;
impl PartialEq<&str> for TrainStationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAIN_STATION_IRI_HTTP || *other == TRAIN_STATION_IRI_HTTPS
	}
}
impl PartialEq<TrainStationIri> for &str {
	fn eq(&self, other: &TrainStationIri) -> bool {
		*self == TRAIN_STATION_IRI_HTTP || *self == TRAIN_STATION_IRI_HTTPS
	}
}
pub struct TrainStationIriOrLabel;
impl PartialEq<&str> for TrainStationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrainStationIri || *other == TRAIN_STATION_LABEL
	}
}
impl PartialEq<TrainStationIriOrLabel> for &str {
	fn eq(&self, other: &TrainStationIriOrLabel) -> bool {
		*self == TrainStationIri || *self == TRAIN_STATION_LABEL
	}
}
