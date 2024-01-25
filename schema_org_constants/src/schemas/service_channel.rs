/// <https://schema.org/ServiceChannel>
pub const SERVICE_CHANNEL_IRI_HTTP: &str = "http://schema.org/ServiceChannel";
/// <https://schema.org/ServiceChannel>
pub const SERVICE_CHANNEL_IRI_HTTPS: &str = "https://schema.org/ServiceChannel";
/// <https://schema.org/ServiceChannel>
pub const SERVICE_CHANNEL_LABEL: &str = "ServiceChannel";
pub struct ServiceChannelIri;
impl PartialEq<&str> for ServiceChannelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_CHANNEL_IRI_HTTP || *other == SERVICE_CHANNEL_IRI_HTTPS
	}
}
impl PartialEq<ServiceChannelIri> for &str {
	fn eq(&self, other: &ServiceChannelIri) -> bool {
		*self == SERVICE_CHANNEL_IRI_HTTP || *self == SERVICE_CHANNEL_IRI_HTTPS
	}
}
pub struct ServiceChannelIriOrLabel;
impl PartialEq<&str> for ServiceChannelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceChannelIri || *other == SERVICE_CHANNEL_LABEL
	}
}
impl PartialEq<ServiceChannelIriOrLabel> for &str {
	fn eq(&self, other: &ServiceChannelIriOrLabel) -> bool {
		*self == ServiceChannelIri || *self == SERVICE_CHANNEL_LABEL
	}
}
