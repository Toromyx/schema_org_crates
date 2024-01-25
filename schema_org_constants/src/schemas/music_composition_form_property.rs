/// <https://schema.org/musicCompositionForm>
pub const MUSIC_COMPOSITION_FORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/musicCompositionForm";
/// <https://schema.org/musicCompositionForm>
pub const MUSIC_COMPOSITION_FORM_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/musicCompositionForm";
/// <https://schema.org/musicCompositionForm>
pub const MUSIC_COMPOSITION_FORM_PROPERTY_LABEL: &str = "musicCompositionForm";
pub struct MusicCompositionFormPropertyIri;
impl PartialEq<&str> for MusicCompositionFormPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_COMPOSITION_FORM_PROPERTY_IRI_HTTP
			|| *other == MUSIC_COMPOSITION_FORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MusicCompositionFormPropertyIri> for &str {
	fn eq(&self, other: &MusicCompositionFormPropertyIri) -> bool {
		*self == MUSIC_COMPOSITION_FORM_PROPERTY_IRI_HTTP
			|| *self == MUSIC_COMPOSITION_FORM_PROPERTY_IRI_HTTPS
	}
}
pub struct MusicCompositionFormPropertyIriOrLabel;
impl PartialEq<&str> for MusicCompositionFormPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicCompositionFormPropertyIri || *other == MUSIC_COMPOSITION_FORM_PROPERTY_LABEL
	}
}
impl PartialEq<MusicCompositionFormPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MusicCompositionFormPropertyIriOrLabel) -> bool {
		*self == MusicCompositionFormPropertyIri || *self == MUSIC_COMPOSITION_FORM_PROPERTY_LABEL
	}
}
