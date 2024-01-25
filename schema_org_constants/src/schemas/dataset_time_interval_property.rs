/// <https://schema.org/datasetTimeInterval>
#[deprecated = "This schema is superseded by <https://schema.org/temporalCoverage>."]
pub const DATASET_TIME_INTERVAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/datasetTimeInterval";
/// <https://schema.org/datasetTimeInterval>
#[deprecated = "This schema is superseded by <https://schema.org/temporalCoverage>."]
pub const DATASET_TIME_INTERVAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/datasetTimeInterval";
/// <https://schema.org/datasetTimeInterval>
#[deprecated = "This schema is superseded by <https://schema.org/temporalCoverage>."]
pub const DATASET_TIME_INTERVAL_PROPERTY_LABEL: &str = "datasetTimeInterval";
pub struct DatasetTimeIntervalPropertyIri;
impl PartialEq<&str> for DatasetTimeIntervalPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATASET_TIME_INTERVAL_PROPERTY_IRI_HTTP
			|| *other == DATASET_TIME_INTERVAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DatasetTimeIntervalPropertyIri> for &str {
	fn eq(&self, other: &DatasetTimeIntervalPropertyIri) -> bool {
		*self == DATASET_TIME_INTERVAL_PROPERTY_IRI_HTTP
			|| *self == DATASET_TIME_INTERVAL_PROPERTY_IRI_HTTPS
	}
}
pub struct DatasetTimeIntervalPropertyIriOrLabel;
impl PartialEq<&str> for DatasetTimeIntervalPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DatasetTimeIntervalPropertyIri || *other == DATASET_TIME_INTERVAL_PROPERTY_LABEL
	}
}
impl PartialEq<DatasetTimeIntervalPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DatasetTimeIntervalPropertyIriOrLabel) -> bool {
		*self == DatasetTimeIntervalPropertyIri || *self == DATASET_TIME_INTERVAL_PROPERTY_LABEL
	}
}
