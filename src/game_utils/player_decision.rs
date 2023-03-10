pub enum PlayerDecision {
    Build,
    Mine,
    Attack,
}

impl TryFrom<String> for PlayerDecision {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str().trim() {
            "build" => Ok(PlayerDecision::Build),
            "mine" => Ok(PlayerDecision::Mine),
            "attack" => Ok(PlayerDecision::Attack),
            _ => Err(()),
        }
    }
}
