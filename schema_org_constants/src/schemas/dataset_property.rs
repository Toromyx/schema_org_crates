/// <https://schema.org/dataset>
pub const DATASET_PROPERTY_IRI_HTTP: &str = "http://schema.org/dataset";
/// <https://schema.org/dataset>
pub const DATASET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dataset";
/// <https://schema.org/dataset>
pub const DATASET_PROPERTY_LABEL: &str = "dataset";
pub struct DatasetPropertyIri;
impl PartialEq<&str> for DatasetPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATASET_PROPERTY_IRI_HTTP || *other == DATASET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DatasetPropertyIri> for &str {
	fn eq(&self, other: &DatasetPropertyIri) -> bool {
		*self == DATASET_PROPERTY_IRI_HTTP || *self == DATASET_PROPERTY_IRI_HTTPS
	}
}
pub struct DatasetPropertyIriOrLabel;
impl PartialEq<&str> for DatasetPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DatasetPropertyIri || *other == DATASET_PROPERTY_LABEL
	}
}
impl PartialEq<DatasetPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DatasetPropertyIriOrLabel) -> bool {
		*self == DatasetPropertyIri || *self == DATASET_PROPERTY_LABEL
	}
}
