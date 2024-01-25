/// <https://schema.org/LandmarksOrHistoricalBuildings>
pub const LANDMARKS_OR_HISTORICAL_BUILDINGS_IRI_HTTP: &str =
	"http://schema.org/LandmarksOrHistoricalBuildings";
/// <https://schema.org/LandmarksOrHistoricalBuildings>
pub const LANDMARKS_OR_HISTORICAL_BUILDINGS_IRI_HTTPS: &str =
	"https://schema.org/LandmarksOrHistoricalBuildings";
/// <https://schema.org/LandmarksOrHistoricalBuildings>
pub const LANDMARKS_OR_HISTORICAL_BUILDINGS_LABEL: &str = "LandmarksOrHistoricalBuildings";
pub struct LandmarksOrHistoricalBuildingsIri;
impl PartialEq<&str> for LandmarksOrHistoricalBuildingsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LANDMARKS_OR_HISTORICAL_BUILDINGS_IRI_HTTP
			|| *other == LANDMARKS_OR_HISTORICAL_BUILDINGS_IRI_HTTPS
	}
}
impl PartialEq<LandmarksOrHistoricalBuildingsIri> for &str {
	fn eq(&self, other: &LandmarksOrHistoricalBuildingsIri) -> bool {
		*self == LANDMARKS_OR_HISTORICAL_BUILDINGS_IRI_HTTP
			|| *self == LANDMARKS_OR_HISTORICAL_BUILDINGS_IRI_HTTPS
	}
}
pub struct LandmarksOrHistoricalBuildingsIriOrLabel;
impl PartialEq<&str> for LandmarksOrHistoricalBuildingsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LandmarksOrHistoricalBuildingsIri
			|| *other == LANDMARKS_OR_HISTORICAL_BUILDINGS_LABEL
	}
}
impl PartialEq<LandmarksOrHistoricalBuildingsIriOrLabel> for &str {
	fn eq(&self, other: &LandmarksOrHistoricalBuildingsIriOrLabel) -> bool {
		*self == LandmarksOrHistoricalBuildingsIri
			|| *self == LANDMARKS_OR_HISTORICAL_BUILDINGS_LABEL
	}
}
