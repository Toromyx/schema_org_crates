/// <https://schema.org/processingTime>
pub const PROCESSING_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/processingTime";
/// <https://schema.org/processingTime>
pub const PROCESSING_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/processingTime";
/// <https://schema.org/processingTime>
pub const PROCESSING_TIME_PROPERTY_LABEL: &str = "processingTime";
pub struct ProcessingTimePropertyIri;
impl PartialEq<&str> for ProcessingTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROCESSING_TIME_PROPERTY_IRI_HTTP || *other == PROCESSING_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProcessingTimePropertyIri> for &str {
	fn eq(&self, other: &ProcessingTimePropertyIri) -> bool {
		*self == PROCESSING_TIME_PROPERTY_IRI_HTTP || *self == PROCESSING_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct ProcessingTimePropertyIriOrLabel;
impl PartialEq<&str> for ProcessingTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProcessingTimePropertyIri || *other == PROCESSING_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<ProcessingTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProcessingTimePropertyIriOrLabel) -> bool {
		*self == ProcessingTimePropertyIri || *self == PROCESSING_TIME_PROPERTY_LABEL
	}
}
