/// <https://schema.org/DataDrivenMediaDigitalSource>
pub const DATA_DRIVEN_MEDIA_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/DataDrivenMediaDigitalSource";
/// <https://schema.org/DataDrivenMediaDigitalSource>
pub const DATA_DRIVEN_MEDIA_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/DataDrivenMediaDigitalSource";
/// <https://schema.org/DataDrivenMediaDigitalSource>
pub const DATA_DRIVEN_MEDIA_DIGITAL_SOURCE_LABEL: &str = "DataDrivenMediaDigitalSource";
pub struct DataDrivenMediaDigitalSourceIri;
impl PartialEq<&str> for DataDrivenMediaDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATA_DRIVEN_MEDIA_DIGITAL_SOURCE_IRI_HTTP
			|| *other == DATA_DRIVEN_MEDIA_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<DataDrivenMediaDigitalSourceIri> for &str {
	fn eq(&self, other: &DataDrivenMediaDigitalSourceIri) -> bool {
		*self == DATA_DRIVEN_MEDIA_DIGITAL_SOURCE_IRI_HTTP
			|| *self == DATA_DRIVEN_MEDIA_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct DataDrivenMediaDigitalSourceIriOrLabel;
impl PartialEq<&str> for DataDrivenMediaDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DataDrivenMediaDigitalSourceIri
			|| *other == DATA_DRIVEN_MEDIA_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<DataDrivenMediaDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &DataDrivenMediaDigitalSourceIriOrLabel) -> bool {
		*self == DataDrivenMediaDigitalSourceIri || *self == DATA_DRIVEN_MEDIA_DIGITAL_SOURCE_LABEL
	}
}
