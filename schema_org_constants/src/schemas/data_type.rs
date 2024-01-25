/// <https://schema.org/DataType>
pub const DATA_TYPE_IRI_HTTP: &str = "http://schema.org/DataType";
/// <https://schema.org/DataType>
pub const DATA_TYPE_IRI_HTTPS: &str = "https://schema.org/DataType";
/// <https://schema.org/DataType>
pub const DATA_TYPE_LABEL: &str = "DataType";
pub struct DataTypeIri;
impl PartialEq<&str> for DataTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATA_TYPE_IRI_HTTP || *other == DATA_TYPE_IRI_HTTPS
	}
}
impl PartialEq<DataTypeIri> for &str {
	fn eq(&self, other: &DataTypeIri) -> bool {
		*self == DATA_TYPE_IRI_HTTP || *self == DATA_TYPE_IRI_HTTPS
	}
}
pub struct DataTypeIriOrLabel;
impl PartialEq<&str> for DataTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DataTypeIri || *other == DATA_TYPE_LABEL
	}
}
impl PartialEq<DataTypeIriOrLabel> for &str {
	fn eq(&self, other: &DataTypeIriOrLabel) -> bool {
		*self == DataTypeIri || *self == DATA_TYPE_LABEL
	}
}
