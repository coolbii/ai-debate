import type { DebateSide } from '@agora/shared-types';

export type SpeakerSeat = 'first' | 'second' | 'third';

export type TurnKind = 'speech' | 'question' | 'closing';

export type TurnSpeaker = {
  side: DebateSide;
  seat: SpeakerSeat;
};

export type TurnPlan = {
  index: number;
  phase: string;
  kind: TurnKind;
  speaker?: TurnSpeaker;
  target?: TurnSpeaker;
  note?: string;
};

export const OREGON_MVP_TURN_PLAN: TurnPlan[] = [
  {
    index: 1,
    phase: 'affirmative-first-constructive',
    kind: 'speech',
    speaker: { side: 'affirmative', seat: 'first' },
  },
  {
    index: 2,
    phase: 'negative-second-cross-affirmative-first',
    kind: 'question',
    speaker: { side: 'negative', seat: 'second' },
    target: { side: 'affirmative', seat: 'first' },
  },
  {
    index: 3,
    phase: 'negative-first-constructive',
    kind: 'speech',
    speaker: { side: 'negative', seat: 'first' },
  },
  {
    index: 4,
    phase: 'affirmative-third-cross-negative-first',
    kind: 'question',
    speaker: { side: 'affirmative', seat: 'third' },
    target: { side: 'negative', seat: 'first' },
  },
  {
    index: 5,
    phase: 'affirmative-second-constructive',
    kind: 'speech',
    speaker: { side: 'affirmative', seat: 'second' },
  },
  {
    index: 6,
    phase: 'negative-third-cross-affirmative-second',
    kind: 'question',
    speaker: { side: 'negative', seat: 'third' },
    target: { side: 'affirmative', seat: 'second' },
  },
  {
    index: 7,
    phase: 'negative-second-constructive',
    kind: 'speech',
    speaker: { side: 'negative', seat: 'second' },
  },
  {
    index: 8,
    phase: 'affirmative-first-cross-negative-second',
    kind: 'question',
    speaker: { side: 'affirmative', seat: 'first' },
    target: { side: 'negative', seat: 'second' },
  },
  {
    index: 9,
    phase: 'affirmative-third-constructive',
    kind: 'speech',
    speaker: { side: 'affirmative', seat: 'third' },
  },
  {
    index: 10,
    phase: 'negative-first-cross-affirmative-third',
    kind: 'question',
    speaker: { side: 'negative', seat: 'first' },
    target: { side: 'affirmative', seat: 'third' },
  },
  {
    index: 11,
    phase: 'negative-third-constructive',
    kind: 'speech',
    speaker: { side: 'negative', seat: 'third' },
  },
  {
    index: 12,
    phase: 'affirmative-second-cross-negative-third',
    kind: 'question',
    speaker: { side: 'affirmative', seat: 'second' },
    target: { side: 'negative', seat: 'third' },
  },
  {
    index: 13,
    phase: 'closing',
    kind: 'closing',
    note: 'Closing sequence is fixed by orchestrator configuration in MVP mode.',
  },
];

export function getPlannedTurn(index: number): TurnPlan | undefined {
  return OREGON_MVP_TURN_PLAN.find((turn) => turn.index === index);
}

export function isCrossExamination(turn: TurnPlan): boolean {
  return turn.kind === 'question';
}
