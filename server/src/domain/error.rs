use snowy_model::{MemberId, TeamId};

#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum Error {
    #[error("Member with email '{0}' already exists")]
    MemberAlreadyExists(String),
    #[error("Member '{0:?}' is not in team '{1:?}'")]
    MemberNotFoundInTeam(MemberId, TeamId),
}
