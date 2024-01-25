/// <https://schema.org/Dataset>
pub const DATASET_IRI_HTTP: &str = "http://schema.org/Dataset";
/// <https://schema.org/Dataset>
pub const DATASET_IRI_HTTPS: &str = "https://schema.org/Dataset";
/// <https://schema.org/Dataset>
pub const DATASET_LABEL: &str = "Dataset";
pub struct DatasetIri;
impl PartialEq<&str> for DatasetIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATASET_IRI_HTTP || *other == DATASET_IRI_HTTPS
	}
}
impl PartialEq<DatasetIri> for &str {
	fn eq(&self, other: &DatasetIri) -> bool {
		*self == DATASET_IRI_HTTP || *self == DATASET_IRI_HTTPS
	}
}
pub struct DatasetIriOrLabel;
impl PartialEq<&str> for DatasetIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DatasetIri || *other == DATASET_LABEL
	}
}
impl PartialEq<DatasetIriOrLabel> for &str {
	fn eq(&self, other: &DatasetIriOrLabel) -> bool {
		*self == DatasetIri || *self == DATASET_LABEL
	}
}
