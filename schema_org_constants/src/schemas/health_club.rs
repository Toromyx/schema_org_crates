/// <https://schema.org/HealthClub>
pub const HEALTH_CLUB_IRI_HTTP: &str = "http://schema.org/HealthClub";
/// <https://schema.org/HealthClub>
pub const HEALTH_CLUB_IRI_HTTPS: &str = "https://schema.org/HealthClub";
/// <https://schema.org/HealthClub>
pub const HEALTH_CLUB_LABEL: &str = "HealthClub";
pub struct HealthClubIri;
impl PartialEq<&str> for HealthClubIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_CLUB_IRI_HTTP || *other == HEALTH_CLUB_IRI_HTTPS
	}
}
impl PartialEq<HealthClubIri> for &str {
	fn eq(&self, other: &HealthClubIri) -> bool {
		*self == HEALTH_CLUB_IRI_HTTP || *self == HEALTH_CLUB_IRI_HTTPS
	}
}
pub struct HealthClubIriOrLabel;
impl PartialEq<&str> for HealthClubIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthClubIri || *other == HEALTH_CLUB_LABEL
	}
}
impl PartialEq<HealthClubIriOrLabel> for &str {
	fn eq(&self, other: &HealthClubIriOrLabel) -> bool {
		*self == HealthClubIri || *self == HEALTH_CLUB_LABEL
	}
}
