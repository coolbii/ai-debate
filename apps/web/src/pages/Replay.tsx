import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useDebateStore } from '../store/debate';
import { DebateEvent } from '../api/client';
import styles from './Replay.module.css';

export function Replay() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const session = useDebateStore((s) => s.session);
  const agents = useDebateStore((s) => s.agents);
  const events = useDebateStore((s) => s.events);
  const isLoading = useDebateStore((s) => s.isLoading);
  const loadSession = useDebateStore((s) => s.loadSession);

  const [cursor, setCursor] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);

  useEffect(() => {
    if (id) loadSession(id);
  }, [id]);

  const speechEvents = events.filter((e) => e.kind !== 'system');

  useEffect(() => {
    if (!isPlaying) return;
    if (cursor >= speechEvents.length) {
      setIsPlaying(false);
      return;
    }
    const t = setTimeout(() => setCursor((c) => c + 1), 1500);
    return () => clearTimeout(t);
  }, [isPlaying, cursor, speechEvents.length]);

  function agentName(speakerId: string) {
    if (speakerId === 'system') return 'System';
    if (speakerId === 'judge') return 'Judge';
    return agents.find((a) => a.id === speakerId)?.displayName ?? speakerId;
  }

  function agentSide(speakerId: string) {
    return agents.find((a) => a.id === speakerId)?.side ?? 'system';
  }

  const visible = speechEvents.slice(0, cursor);

  if (isLoading && !session) return <div className={styles.loading}>Loading…</div>;
  if (!session) return <div className={styles.loading}>Session not found</div>;

  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <button className={styles.back} onClick={() => navigate(`/session/${id}`)}>← Back</button>
        <h1 className={styles.title}>Replay — {session.motion}</h1>
      </header>

      <div className={styles.controls}>
        <button
          className={styles.btn}
          onClick={() => { setCursor(0); setIsPlaying(false); }}
        >
          Reset
        </button>
        <button
          className={styles.btn}
          onClick={() => setCursor((c) => Math.max(0, c - 1))}
        >
          Prev
        </button>
        <button
          className={`${styles.btn} ${styles.primary}`}
          onClick={() => setIsPlaying((p) => !p)}
        >
          {isPlaying ? 'Pause' : 'Play'}
        </button>
        <button
          className={styles.btn}
          onClick={() => setCursor((c) => Math.min(speechEvents.length, c + 1))}
        >
          Next
        </button>
        <span className={styles.progress}>{cursor} / {speechEvents.length}</span>
      </div>

      {/* Timeline scrubber */}
      <input
        className={styles.scrubber}
        type="range"
        min={0}
        max={speechEvents.length}
        value={cursor}
        onChange={(e) => { setIsPlaying(false); setCursor(Number(e.target.value)); }}
      />

      <div className={styles.transcript}>
        {visible.map((event: DebateEvent) => (
          <div key={event.id} className={`${styles.entry} ${styles[agentSide(event.speakerId)]}`}>
            <div className={styles.meta}>
              <span className={styles.name}>{agentName(event.speakerId)}</span>
              <span className={styles.phase}>{event.phase}</span>
            </div>
            <p className={styles.content}>{event.content}</p>
          </div>
        ))}
        {cursor < speechEvents.length && (
          <div className={styles.upcoming}>
            Next: {agentName(speechEvents[cursor]?.speakerId)} — {speechEvents[cursor]?.phase}
          </div>
        )}
      </div>
    </div>
  );
}
