/// <https://schema.org/sdPublisher>
pub const SD_PUBLISHER_PROPERTY_IRI_HTTP: &str = "http://schema.org/sdPublisher";
/// <https://schema.org/sdPublisher>
pub const SD_PUBLISHER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sdPublisher";
/// <https://schema.org/sdPublisher>
pub const SD_PUBLISHER_PROPERTY_LABEL: &str = "sdPublisher";
pub struct SdPublisherPropertyIri;
impl PartialEq<&str> for SdPublisherPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SD_PUBLISHER_PROPERTY_IRI_HTTP || *other == SD_PUBLISHER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SdPublisherPropertyIri> for &str {
	fn eq(&self, other: &SdPublisherPropertyIri) -> bool {
		*self == SD_PUBLISHER_PROPERTY_IRI_HTTP || *self == SD_PUBLISHER_PROPERTY_IRI_HTTPS
	}
}
pub struct SdPublisherPropertyIriOrLabel;
impl PartialEq<&str> for SdPublisherPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SdPublisherPropertyIri || *other == SD_PUBLISHER_PROPERTY_LABEL
	}
}
impl PartialEq<SdPublisherPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SdPublisherPropertyIriOrLabel) -> bool {
		*self == SdPublisherPropertyIri || *self == SD_PUBLISHER_PROPERTY_LABEL
	}
}
