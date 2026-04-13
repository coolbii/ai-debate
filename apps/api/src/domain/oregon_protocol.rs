/// Oregon-style 3v3 debate turn plan (matches TypeScript OREGON_MVP_TURN_PLAN)
#[derive(Debug, Clone)]
pub struct TurnPlan {
    pub index: u32,
    pub phase: &'static str,
    pub kind: TurnKind,
    pub speaker_side: Option<&'static str>,  // "affirmative" | "negative"
    pub speaker_seat: Option<&'static str>,  // "first" | "second" | "third"
    pub target_side: Option<&'static str>,
    pub target_seat: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnKind {
    Speech,
    Question,
    Closing,
}

pub const OREGON_TURNS: &[TurnPlan] = &[
    TurnPlan { index: 1,  phase: "affirmative-first-constructive",         kind: TurnKind::Speech,   speaker_side: Some("affirmative"), speaker_seat: Some("first"),  target_side: None,              target_seat: None },
    TurnPlan { index: 2,  phase: "negative-second-cross-affirmative-first", kind: TurnKind::Question, speaker_side: Some("negative"),    speaker_seat: Some("second"), target_side: Some("affirmative"), target_seat: Some("first") },
    TurnPlan { index: 3,  phase: "negative-first-constructive",            kind: TurnKind::Speech,   speaker_side: Some("negative"),    speaker_seat: Some("first"),  target_side: None,              target_seat: None },
    TurnPlan { index: 4,  phase: "affirmative-third-cross-negative-first", kind: TurnKind::Question, speaker_side: Some("affirmative"), speaker_seat: Some("third"),  target_side: Some("negative"),   target_seat: Some("first") },
    TurnPlan { index: 5,  phase: "affirmative-second-constructive",        kind: TurnKind::Speech,   speaker_side: Some("affirmative"), speaker_seat: Some("second"), target_side: None,              target_seat: None },
    TurnPlan { index: 6,  phase: "negative-third-cross-affirmative-second",kind: TurnKind::Question, speaker_side: Some("negative"),    speaker_seat: Some("third"),  target_side: Some("affirmative"), target_seat: Some("second") },
    TurnPlan { index: 7,  phase: "negative-second-constructive",           kind: TurnKind::Speech,   speaker_side: Some("negative"),    speaker_seat: Some("second"), target_side: None,              target_seat: None },
    TurnPlan { index: 8,  phase: "affirmative-first-cross-negative-second",kind: TurnKind::Question, speaker_side: Some("affirmative"), speaker_seat: Some("first"),  target_side: Some("negative"),   target_seat: Some("second") },
    TurnPlan { index: 9,  phase: "affirmative-third-constructive",         kind: TurnKind::Speech,   speaker_side: Some("affirmative"), speaker_seat: Some("third"),  target_side: None,              target_seat: None },
    TurnPlan { index: 10, phase: "negative-first-cross-affirmative-third", kind: TurnKind::Question, speaker_side: Some("negative"),    speaker_seat: Some("first"),  target_side: Some("affirmative"), target_seat: Some("third") },
    TurnPlan { index: 11, phase: "negative-third-constructive",            kind: TurnKind::Speech,   speaker_side: Some("negative"),    speaker_seat: Some("third"),  target_side: None,              target_seat: None },
    TurnPlan { index: 12, phase: "affirmative-second-cross-negative-third",kind: TurnKind::Question, speaker_side: Some("affirmative"), speaker_seat: Some("second"), target_side: Some("negative"),   target_seat: Some("third") },
    TurnPlan { index: 13, phase: "closing",                                kind: TurnKind::Closing,  speaker_side: None,               speaker_seat: None,           target_side: None,              target_seat: None },
];

pub fn get_turn(index: u32) -> Option<&'static TurnPlan> {
    OREGON_TURNS.iter().find(|t| t.index == index)
}
