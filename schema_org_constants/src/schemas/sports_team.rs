/// <https://schema.org/SportsTeam>
pub const SPORTS_TEAM_IRI_HTTP: &str = "http://schema.org/SportsTeam";
/// <https://schema.org/SportsTeam>
pub const SPORTS_TEAM_IRI_HTTPS: &str = "https://schema.org/SportsTeam";
/// <https://schema.org/SportsTeam>
pub const SPORTS_TEAM_LABEL: &str = "SportsTeam";
pub struct SportsTeamIri;
impl PartialEq<&str> for SportsTeamIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORTS_TEAM_IRI_HTTP || *other == SPORTS_TEAM_IRI_HTTPS
	}
}
impl PartialEq<SportsTeamIri> for &str {
	fn eq(&self, other: &SportsTeamIri) -> bool {
		*self == SPORTS_TEAM_IRI_HTTP || *self == SPORTS_TEAM_IRI_HTTPS
	}
}
pub struct SportsTeamIriOrLabel;
impl PartialEq<&str> for SportsTeamIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportsTeamIri || *other == SPORTS_TEAM_LABEL
	}
}
impl PartialEq<SportsTeamIriOrLabel> for &str {
	fn eq(&self, other: &SportsTeamIriOrLabel) -> bool {
		*self == SportsTeamIri || *self == SPORTS_TEAM_LABEL
	}
}
