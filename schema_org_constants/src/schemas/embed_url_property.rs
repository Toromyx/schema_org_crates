/// <https://schema.org/embedUrl>
pub const EMBED_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/embedUrl";
/// <https://schema.org/embedUrl>
pub const EMBED_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/embedUrl";
/// <https://schema.org/embedUrl>
pub const EMBED_URL_PROPERTY_LABEL: &str = "embedUrl";
pub struct EmbedUrlPropertyIri;
impl PartialEq<&str> for EmbedUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMBED_URL_PROPERTY_IRI_HTTP || *other == EMBED_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EmbedUrlPropertyIri> for &str {
	fn eq(&self, other: &EmbedUrlPropertyIri) -> bool {
		*self == EMBED_URL_PROPERTY_IRI_HTTP || *self == EMBED_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct EmbedUrlPropertyIriOrLabel;
impl PartialEq<&str> for EmbedUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmbedUrlPropertyIri || *other == EMBED_URL_PROPERTY_LABEL
	}
}
impl PartialEq<EmbedUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EmbedUrlPropertyIriOrLabel) -> bool {
		*self == EmbedUrlPropertyIri || *self == EMBED_URL_PROPERTY_LABEL
	}
}
