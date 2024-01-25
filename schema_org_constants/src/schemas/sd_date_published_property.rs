/// <https://schema.org/sdDatePublished>
pub const SD_DATE_PUBLISHED_PROPERTY_IRI_HTTP: &str = "http://schema.org/sdDatePublished";
/// <https://schema.org/sdDatePublished>
pub const SD_DATE_PUBLISHED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sdDatePublished";
/// <https://schema.org/sdDatePublished>
pub const SD_DATE_PUBLISHED_PROPERTY_LABEL: &str = "sdDatePublished";
pub struct SdDatePublishedPropertyIri;
impl PartialEq<&str> for SdDatePublishedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SD_DATE_PUBLISHED_PROPERTY_IRI_HTTP
			|| *other == SD_DATE_PUBLISHED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SdDatePublishedPropertyIri> for &str {
	fn eq(&self, other: &SdDatePublishedPropertyIri) -> bool {
		*self == SD_DATE_PUBLISHED_PROPERTY_IRI_HTTP
			|| *self == SD_DATE_PUBLISHED_PROPERTY_IRI_HTTPS
	}
}
pub struct SdDatePublishedPropertyIriOrLabel;
impl PartialEq<&str> for SdDatePublishedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SdDatePublishedPropertyIri || *other == SD_DATE_PUBLISHED_PROPERTY_LABEL
	}
}
impl PartialEq<SdDatePublishedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SdDatePublishedPropertyIriOrLabel) -> bool {
		*self == SdDatePublishedPropertyIri || *self == SD_DATE_PUBLISHED_PROPERTY_LABEL
	}
}
