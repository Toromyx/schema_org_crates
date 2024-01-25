/// <https://schema.org/subtitleLanguage>
pub const SUBTITLE_LANGUAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/subtitleLanguage";
/// <https://schema.org/subtitleLanguage>
pub const SUBTITLE_LANGUAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subtitleLanguage";
/// <https://schema.org/subtitleLanguage>
pub const SUBTITLE_LANGUAGE_PROPERTY_LABEL: &str = "subtitleLanguage";
pub struct SubtitleLanguagePropertyIri;
impl PartialEq<&str> for SubtitleLanguagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUBTITLE_LANGUAGE_PROPERTY_IRI_HTTP
			|| *other == SUBTITLE_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubtitleLanguagePropertyIri> for &str {
	fn eq(&self, other: &SubtitleLanguagePropertyIri) -> bool {
		*self == SUBTITLE_LANGUAGE_PROPERTY_IRI_HTTP
			|| *self == SUBTITLE_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct SubtitleLanguagePropertyIriOrLabel;
impl PartialEq<&str> for SubtitleLanguagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubtitleLanguagePropertyIri || *other == SUBTITLE_LANGUAGE_PROPERTY_LABEL
	}
}
impl PartialEq<SubtitleLanguagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubtitleLanguagePropertyIriOrLabel) -> bool {
		*self == SubtitleLanguagePropertyIri || *self == SUBTITLE_LANGUAGE_PROPERTY_LABEL
	}
}
