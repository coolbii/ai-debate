export type DebateFormat = 'oregon-3v3';

export type DebateMode = 'formal' | 'demo';

export type SessionStatus = 'draft' | 'running' | 'paused' | 'finished';

export type DebateSide = 'affirmative' | 'negative';

export type DebaterRole = 'first' | 'second' | 'third' | 'closer';

export type EventKind = 'speech' | 'question' | 'answer' | 'system' | 'judge_note';

export type DebateSession = {
  id: string;
  motion: string;
  definition: string;
  format: DebateFormat;
  mode: DebateMode;
  status: SessionStatus;
  currentRound: number;
  currentPhase: string;
  affirmativeTeamId: string;
  negativeTeamId: string;
  judgeId: string;
  createdAt: string;
  updatedAt: string;
};

export type DebaterAgent = {
  id: string;
  sessionId: string;
  side: DebateSide;
  role: DebaterRole;
  displayName: string;
  persona: string;
  objective: string;
  style: string;
  model: string;
};

export type DebateEvent = {
  id: string;
  sessionId: string;
  round: number;
  phase: string;
  speakerId: string;
  targetId?: string;
  kind: EventKind;
  content: string;
  meta?: Record<string, unknown>;
  startedAt: string;
  endedAt?: string;
};

export type JudgeDecision = {
  sessionId: string;
  winner: DebateSide;
  reasoning: string;
  notesByRound: Array<{
    round: number;
    note: string;
  }>;
  decidedAt: string;
};
