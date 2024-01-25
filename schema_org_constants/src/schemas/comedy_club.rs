/// <https://schema.org/ComedyClub>
pub const COMEDY_CLUB_IRI_HTTP: &str = "http://schema.org/ComedyClub";
/// <https://schema.org/ComedyClub>
pub const COMEDY_CLUB_IRI_HTTPS: &str = "https://schema.org/ComedyClub";
/// <https://schema.org/ComedyClub>
pub const COMEDY_CLUB_LABEL: &str = "ComedyClub";
pub struct ComedyClubIri;
impl PartialEq<&str> for ComedyClubIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMEDY_CLUB_IRI_HTTP || *other == COMEDY_CLUB_IRI_HTTPS
	}
}
impl PartialEq<ComedyClubIri> for &str {
	fn eq(&self, other: &ComedyClubIri) -> bool {
		*self == COMEDY_CLUB_IRI_HTTP || *self == COMEDY_CLUB_IRI_HTTPS
	}
}
pub struct ComedyClubIriOrLabel;
impl PartialEq<&str> for ComedyClubIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComedyClubIri || *other == COMEDY_CLUB_LABEL
	}
}
impl PartialEq<ComedyClubIriOrLabel> for &str {
	fn eq(&self, other: &ComedyClubIriOrLabel) -> bool {
		*self == ComedyClubIri || *self == COMEDY_CLUB_LABEL
	}
}
