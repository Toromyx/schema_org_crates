/// <https://schema.org/SportsClub>
pub const SPORTS_CLUB_IRI_HTTP: &str = "http://schema.org/SportsClub";
/// <https://schema.org/SportsClub>
pub const SPORTS_CLUB_IRI_HTTPS: &str = "https://schema.org/SportsClub";
/// <https://schema.org/SportsClub>
pub const SPORTS_CLUB_LABEL: &str = "SportsClub";
pub struct SportsClubIri;
impl PartialEq<&str> for SportsClubIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORTS_CLUB_IRI_HTTP || *other == SPORTS_CLUB_IRI_HTTPS
	}
}
impl PartialEq<SportsClubIri> for &str {
	fn eq(&self, other: &SportsClubIri) -> bool {
		*self == SPORTS_CLUB_IRI_HTTP || *self == SPORTS_CLUB_IRI_HTTPS
	}
}
pub struct SportsClubIriOrLabel;
impl PartialEq<&str> for SportsClubIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportsClubIri || *other == SPORTS_CLUB_LABEL
	}
}
impl PartialEq<SportsClubIriOrLabel> for &str {
	fn eq(&self, other: &SportsClubIriOrLabel) -> bool {
		*self == SportsClubIri || *self == SPORTS_CLUB_LABEL
	}
}
