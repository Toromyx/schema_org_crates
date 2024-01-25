/// <https://schema.org/sportsTeam>
pub const SPORTS_TEAM_PROPERTY_IRI_HTTP: &str = "http://schema.org/sportsTeam";
/// <https://schema.org/sportsTeam>
pub const SPORTS_TEAM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sportsTeam";
/// <https://schema.org/sportsTeam>
pub const SPORTS_TEAM_PROPERTY_LABEL: &str = "sportsTeam";
pub struct SportsTeamPropertyIri;
impl PartialEq<&str> for SportsTeamPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORTS_TEAM_PROPERTY_IRI_HTTP || *other == SPORTS_TEAM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SportsTeamPropertyIri> for &str {
	fn eq(&self, other: &SportsTeamPropertyIri) -> bool {
		*self == SPORTS_TEAM_PROPERTY_IRI_HTTP || *self == SPORTS_TEAM_PROPERTY_IRI_HTTPS
	}
}
pub struct SportsTeamPropertyIriOrLabel;
impl PartialEq<&str> for SportsTeamPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportsTeamPropertyIri || *other == SPORTS_TEAM_PROPERTY_LABEL
	}
}
impl PartialEq<SportsTeamPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SportsTeamPropertyIriOrLabel) -> bool {
		*self == SportsTeamPropertyIri || *self == SPORTS_TEAM_PROPERTY_LABEL
	}
}
