const API_BASE = import.meta.env.VITE_API_URL ?? 'http://localhost:3000';

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    headers: { 'Content-Type': 'application/json', ...init?.headers },
    ...init,
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: res.statusText }));
    throw new Error(body.error ?? `HTTP ${res.status}`);
  }
  // 204 No Content
  if (res.status === 204) return undefined as T;
  return res.json() as Promise<T>;
}

// ── Types matching the Rust backend camelCase output ─────────────────────────

export interface DebateSession {
  id: string;
  motion: string;
  definition: string;
  mode: 'formal' | 'demo';
  status: 'draft' | 'running' | 'paused' | 'finished';
  currentRound: number;
  currentPhase: string;
  model: string;
  createdAt: string;
  updatedAt: string;
}

export interface DebaterAgent {
  id: string;
  sessionId: string;
  side: 'affirmative' | 'negative';
  seat: 'first' | 'second' | 'third';
  displayName: string;
  persona: string;
  objective: string;
  style: string;
  model: string;
}

export interface DebateEvent {
  id: string;
  sessionId: string;
  round: number;
  phase: string;
  speakerId: string;
  targetId?: string;
  kind: 'speech' | 'question' | 'answer' | 'system' | 'judge_note';
  content: string;
  meta?: Record<string, unknown>;
  startedAt: string;
  endedAt?: string;
}

export interface JudgeDecision {
  sessionId: string;
  winner: 'affirmative' | 'negative';
  reasoning: string;
  notesByRound: Array<{ round: number; note: string }>;
  decidedAt: string;
}

// ── API calls ────────────────────────────────────────────────────────────────

export const api = {
  createSession: (body: {
    motion: string;
    definition: string;
    mode: string;
    model: string;
  }) => request<DebateSession>('/sessions', { method: 'POST', body: JSON.stringify(body) }),

  getSession: (id: string) => request<DebateSession>(`/sessions/${id}`),

  getAgents: (id: string) => request<DebaterAgent[]>(`/sessions/${id}/agents`),

  startSession: (id: string) =>
    request<DebateSession>(`/sessions/${id}/start`, { method: 'POST' }),

  pauseSession: (id: string) =>
    request<DebateSession>(`/sessions/${id}/pause`, { method: 'POST' }),

  resumeSession: (id: string) =>
    request<DebateSession>(`/sessions/${id}/resume`, { method: 'POST' }),

  nextTurn: (id: string) =>
    request<DebateEvent>(`/sessions/${id}/next-turn`, { method: 'POST' }),

  declareWinner: (id: string, winner: string, reasoning: string) =>
    request<JudgeDecision>(`/sessions/${id}/declare-winner`, {
      method: 'POST',
      body: JSON.stringify({ winner, reasoning }),
    }),

  getEvents: (id: string) => request<DebateEvent[]>(`/sessions/${id}/events`),

  getJudgeDecision: (id: string) =>
    request<JudgeDecision>(`/sessions/${id}/judge-decision`),

  addJudgeNote: (id: string, round: number, note: string) =>
    request<void>(`/sessions/${id}/judge-notes`, {
      method: 'POST',
      body: JSON.stringify({ round, note }),
    }),
};

// ── Export helpers ──────────────────────────────────────────────────────────

export function downloadTranscript(sessionId: string) {
  window.open(`${API_BASE}/sessions/${sessionId}/export/transcript`, '_blank');
}

export function downloadEvents(sessionId: string) {
  window.open(`${API_BASE}/sessions/${sessionId}/export/events`, '_blank');
}

// ── WebSocket helper ─────────────────────────────────────────────────────────

const WS_BASE = API_BASE.replace(/^http/, 'ws');

export function connectStream(
  sessionId: string,
  onEvent: (event: DebateEvent) => void,
  onClose?: () => void,
): WebSocket {
  const ws = new WebSocket(`${WS_BASE}/sessions/${sessionId}/stream`);
  ws.onmessage = (e) => {
    try {
      onEvent(JSON.parse(e.data) as DebateEvent);
    } catch {
      // ignore malformed
    }
  };
  ws.onclose = () => onClose?.();
  return ws;
}
