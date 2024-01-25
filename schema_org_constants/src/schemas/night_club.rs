/// <https://schema.org/NightClub>
pub const NIGHT_CLUB_IRI_HTTP: &str = "http://schema.org/NightClub";
/// <https://schema.org/NightClub>
pub const NIGHT_CLUB_IRI_HTTPS: &str = "https://schema.org/NightClub";
/// <https://schema.org/NightClub>
pub const NIGHT_CLUB_LABEL: &str = "NightClub";
pub struct NightClubIri;
impl PartialEq<&str> for NightClubIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NIGHT_CLUB_IRI_HTTP || *other == NIGHT_CLUB_IRI_HTTPS
	}
}
impl PartialEq<NightClubIri> for &str {
	fn eq(&self, other: &NightClubIri) -> bool {
		*self == NIGHT_CLUB_IRI_HTTP || *self == NIGHT_CLUB_IRI_HTTPS
	}
}
pub struct NightClubIriOrLabel;
impl PartialEq<&str> for NightClubIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NightClubIri || *other == NIGHT_CLUB_LABEL
	}
}
impl PartialEq<NightClubIriOrLabel> for &str {
	fn eq(&self, other: &NightClubIriOrLabel) -> bool {
		*self == NightClubIri || *self == NIGHT_CLUB_LABEL
	}
}
