/// <https://schema.org/diseaseSpreadStatistics>
pub const DISEASE_SPREAD_STATISTICS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/diseaseSpreadStatistics";
/// <https://schema.org/diseaseSpreadStatistics>
pub const DISEASE_SPREAD_STATISTICS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/diseaseSpreadStatistics";
/// <https://schema.org/diseaseSpreadStatistics>
pub const DISEASE_SPREAD_STATISTICS_PROPERTY_LABEL: &str = "diseaseSpreadStatistics";
pub struct DiseaseSpreadStatisticsPropertyIri;
impl PartialEq<&str> for DiseaseSpreadStatisticsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISEASE_SPREAD_STATISTICS_PROPERTY_IRI_HTTP
			|| *other == DISEASE_SPREAD_STATISTICS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiseaseSpreadStatisticsPropertyIri> for &str {
	fn eq(&self, other: &DiseaseSpreadStatisticsPropertyIri) -> bool {
		*self == DISEASE_SPREAD_STATISTICS_PROPERTY_IRI_HTTP
			|| *self == DISEASE_SPREAD_STATISTICS_PROPERTY_IRI_HTTPS
	}
}
pub struct DiseaseSpreadStatisticsPropertyIriOrLabel;
impl PartialEq<&str> for DiseaseSpreadStatisticsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiseaseSpreadStatisticsPropertyIri
			|| *other == DISEASE_SPREAD_STATISTICS_PROPERTY_LABEL
	}
}
impl PartialEq<DiseaseSpreadStatisticsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiseaseSpreadStatisticsPropertyIriOrLabel) -> bool {
		*self == DiseaseSpreadStatisticsPropertyIri
			|| *self == DISEASE_SPREAD_STATISTICS_PROPERTY_LABEL
	}
}
