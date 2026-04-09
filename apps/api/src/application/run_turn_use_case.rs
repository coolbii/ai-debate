use crate::domain::debate_round::DebateRound;

pub fn run_turn(round: &DebateRound) -> String {
    format!(
        "Executing round {} in phase {}",
        round.index, round.phase
    )
}
