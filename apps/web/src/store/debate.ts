import { create } from 'zustand';
import {
  api,
  connectStream,
  DebateEvent,
  DebaterAgent,
  DebateSession,
  JudgeDecision,
} from '../api/client';

interface DebateState {
  session: DebateSession | null;
  agents: DebaterAgent[];
  events: DebateEvent[];
  decision: JudgeDecision | null;
  isLoading: boolean;
  isRunningTurn: boolean;
  error: string | null;
  ws: WebSocket | null;

  // Actions
  createSession: (params: {
    motion: string;
    definition: string;
    mode: string;
    model: string;
  }) => Promise<string>; // returns session id

  loadSession: (id: string) => Promise<void>;
  startSession: () => Promise<void>;
  pauseSession: () => Promise<void>;
  resumeSession: () => Promise<void>;
  nextTurn: () => Promise<void>;
  autoRun: (intervalMs?: number) => void;
  stopAutoRun: () => void;
  addJudgeNote: (note: string) => Promise<void>;
  declareWinner: (winner: string, reasoning: string) => Promise<void>;
  connectStream: () => void;
  disconnectStream: () => void;
  clearError: () => void;
}

let autoRunTimer: ReturnType<typeof setTimeout> | null = null;

export const useDebateStore = create<DebateState>((set, get) => ({
  session: null,
  agents: [],
  events: [],
  decision: null,
  isLoading: false,
  isRunningTurn: false,
  error: null,
  ws: null,

  createSession: async (params) => {
    set({ isLoading: true, error: null });
    try {
      const session = await api.createSession(params);
      const agents = await api.getAgents(session.id);
      set({ session, agents, events: [], decision: null, isLoading: false });
      return session.id;
    } catch (e) {
      set({ isLoading: false, error: String(e) });
      throw e;
    }
  },

  loadSession: async (id) => {
    set({ isLoading: true, error: null });
    try {
      const [session, agents, events] = await Promise.all([
        api.getSession(id),
        api.getAgents(id),
        api.getEvents(id),
      ]);
      const decision = await api.getJudgeDecision(id).catch(() => null);
      set({ session, agents, events, decision: decision ?? null, isLoading: false });
    } catch (e) {
      set({ isLoading: false, error: String(e) });
    }
  },

  startSession: async () => {
    const { session } = get();
    if (!session) return;
    set({ isLoading: true, error: null });
    try {
      const updated = await api.startSession(session.id);
      set({ session: updated, isLoading: false });
      get().connectStream();
    } catch (e) {
      set({ isLoading: false, error: String(e) });
    }
  },

  pauseSession: async () => {
    const { session } = get();
    if (!session) return;
    get().stopAutoRun();
    try {
      const updated = await api.pauseSession(session.id);
      set({ session: updated });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  resumeSession: async () => {
    const { session } = get();
    if (!session) return;
    try {
      const updated = await api.resumeSession(session.id);
      set({ session: updated });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  nextTurn: async () => {
    const { session, isRunningTurn } = get();
    if (!session || isRunningTurn) return;
    set({ isRunningTurn: true, error: null });
    try {
      await api.nextTurn(session.id);
      // session state update comes via WS; also refresh session directly
      const updated = await api.getSession(session.id);
      set({ session: updated, isRunningTurn: false });
    } catch (e) {
      set({ isRunningTurn: false, error: String(e) });
    }
  },

  autoRun: (intervalMs = 2000) => {
    const tick = async () => {
      const { session, isRunningTurn } = get();
      if (!session || session.status !== 'running' || isRunningTurn) return;
      await get().nextTurn();
      autoRunTimer = setTimeout(tick, intervalMs);
    };
    tick();
  },

  stopAutoRun: () => {
    if (autoRunTimer) {
      clearTimeout(autoRunTimer);
      autoRunTimer = null;
    }
  },

  addJudgeNote: async (note) => {
    const { session } = get();
    if (!session) return;
    try {
      await api.addJudgeNote(session.id, session.currentRound, note);
    } catch (e) {
      set({ error: String(e) });
    }
  },

  declareWinner: async (winner, reasoning) => {
    const { session } = get();
    if (!session) return;
    get().stopAutoRun();
    try {
      const decision = await api.declareWinner(session.id, winner, reasoning);
      set({ decision });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  connectStream: () => {
    const { session, ws } = get();
    if (!session) return;
    if (ws) ws.close();

    const socket = connectStream(
      session.id,
      (event) => {
        set((state) => ({
          // Deduplicate by id
          events: state.events.some((e) => e.id === event.id)
            ? state.events
            : [...state.events, event],
        }));
      },
      () => set({ ws: null }),
    );
    set({ ws: socket });
  },

  disconnectStream: () => {
    const { ws } = get();
    if (ws) {
      ws.close();
      set({ ws: null });
    }
  },

  clearError: () => set({ error: null }),
}));
