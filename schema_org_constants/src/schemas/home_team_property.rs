/// <https://schema.org/homeTeam>
pub const HOME_TEAM_PROPERTY_IRI_HTTP: &str = "http://schema.org/homeTeam";
/// <https://schema.org/homeTeam>
pub const HOME_TEAM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/homeTeam";
/// <https://schema.org/homeTeam>
pub const HOME_TEAM_PROPERTY_LABEL: &str = "homeTeam";
pub struct HomeTeamPropertyIri;
impl PartialEq<&str> for HomeTeamPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOME_TEAM_PROPERTY_IRI_HTTP || *other == HOME_TEAM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HomeTeamPropertyIri> for &str {
	fn eq(&self, other: &HomeTeamPropertyIri) -> bool {
		*self == HOME_TEAM_PROPERTY_IRI_HTTP || *self == HOME_TEAM_PROPERTY_IRI_HTTPS
	}
}
pub struct HomeTeamPropertyIriOrLabel;
impl PartialEq<&str> for HomeTeamPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HomeTeamPropertyIri || *other == HOME_TEAM_PROPERTY_LABEL
	}
}
impl PartialEq<HomeTeamPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HomeTeamPropertyIriOrLabel) -> bool {
		*self == HomeTeamPropertyIri || *self == HOME_TEAM_PROPERTY_LABEL
	}
}
