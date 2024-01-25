/// <https://schema.org/SizeSystemMetric>
pub const SIZE_SYSTEM_METRIC_IRI_HTTP: &str = "http://schema.org/SizeSystemMetric";
/// <https://schema.org/SizeSystemMetric>
pub const SIZE_SYSTEM_METRIC_IRI_HTTPS: &str = "https://schema.org/SizeSystemMetric";
/// <https://schema.org/SizeSystemMetric>
pub const SIZE_SYSTEM_METRIC_LABEL: &str = "SizeSystemMetric";
pub struct SizeSystemMetricIri;
impl PartialEq<&str> for SizeSystemMetricIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIZE_SYSTEM_METRIC_IRI_HTTP || *other == SIZE_SYSTEM_METRIC_IRI_HTTPS
	}
}
impl PartialEq<SizeSystemMetricIri> for &str {
	fn eq(&self, other: &SizeSystemMetricIri) -> bool {
		*self == SIZE_SYSTEM_METRIC_IRI_HTTP || *self == SIZE_SYSTEM_METRIC_IRI_HTTPS
	}
}
pub struct SizeSystemMetricIriOrLabel;
impl PartialEq<&str> for SizeSystemMetricIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SizeSystemMetricIri || *other == SIZE_SYSTEM_METRIC_LABEL
	}
}
impl PartialEq<SizeSystemMetricIriOrLabel> for &str {
	fn eq(&self, other: &SizeSystemMetricIriOrLabel) -> bool {
		*self == SizeSystemMetricIri || *self == SIZE_SYSTEM_METRIC_LABEL
	}
}
