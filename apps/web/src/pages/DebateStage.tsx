import { useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { DebaterCard } from '@agora/ui';
import { useDebateStore } from '../store/debate';
import { downloadTranscript, downloadEvents } from '../api/client';
import { DefinitionPanel } from '../components/DefinitionPanel';
import { RoundStatusBanner } from '../components/RoundStatusBanner';
import { SpeechPanel } from '../components/SpeechPanel';
import { JudgePanel } from '../components/JudgePanel';
import styles from './DebateStage.module.css';

export function DebateStage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  const session = useDebateStore((s) => s.session);
  const agents = useDebateStore((s) => s.agents);
  const events = useDebateStore((s) => s.events);
  const isLoading = useDebateStore((s) => s.isLoading);
  const isRunningTurn = useDebateStore((s) => s.isRunningTurn);
  const loadSession = useDebateStore((s) => s.loadSession);
  const connectStream = useDebateStore((s) => s.connectStream);
  const disconnectStream = useDebateStore((s) => s.disconnectStream);

  useEffect(() => {
    if (!id) return;
    loadSession(id).then(() => {
      // Connect WS if already running
      const s = useDebateStore.getState().session;
      if (s && (s.status === 'running' || s.status === 'paused')) {
        connectStream();
      }
    });
    return () => disconnectStream();
  }, [id]);

  if (isLoading && !session) {
    return <div className={styles.loading}>Loading session…</div>;
  }

  if (!session) {
    return (
      <div className={styles.loading}>
        Session not found.{' '}
        <button onClick={() => navigate('/create')}>Create new</button>
      </div>
    );
  }

  const affirmativeAgents = agents.filter((a) => a.side === 'affirmative');
  const negativeAgents = agents.filter((a) => a.side === 'negative');

  // Current speaker from the latest non-system event
  const lastEvent = [...events].reverse().find((e) => e.kind !== 'system' && e.kind !== 'judge_note');
  const currentSpeakerName = lastEvent
    ? agents.find((a) => a.id === lastEvent.speakerId)?.displayName
    : undefined;

  function agentStatus(agentId: string): 'standby' | 'speaking' | 'completed' {
    if (lastEvent?.speakerId === agentId) return 'speaking';
    if (events.some((e) => e.speakerId === agentId)) return 'completed';
    return 'standby';
  }

  return (
    <div className={styles.layout}>
      {/* Header */}
      <header className={styles.header}>
        <button className={styles.back} onClick={() => navigate('/create')}>← New</button>
        <RoundStatusBanner
          round={session.currentRound}
          phase={session.currentPhase}
          status={session.status}
          isRunningTurn={isRunningTurn}
          speakerName={currentSpeakerName}
        />
        <button
          className={styles.replayBtn}
          onClick={() => navigate(`/session/${session.id}/replay`)}
        >
          Replay
        </button>
        {events.length > 0 && (
          <>
            <button className={styles.replayBtn} onClick={() => downloadTranscript(session.id)}>
              Export .md
            </button>
            <button className={styles.replayBtn} onClick={() => downloadEvents(session.id)}>
              Export .jsonl
            </button>
          </>
        )}
      </header>

      {/* Definition */}
      <div className={styles.definition}>
        <DefinitionPanel
          motion={session.motion}
          definition={session.definition}
          mode={session.mode}
        />
      </div>

      {/* Main three-column board */}
      <div className={styles.board}>
        {/* Affirmative column */}
        <aside className={styles.teamCol}>
          <h2 className={styles.teamHeading}>Affirmative</h2>
          {affirmativeAgents.map((agent) => (
            <DebaterCard
              key={agent.id}
              agent={agent}
              status={agentStatus(agent.id)}
            />
          ))}
        </aside>

        {/* Centre: transcript */}
        <section className={styles.stage}>
          <SpeechPanel events={events} agents={agents} />
        </section>

        {/* Right: negative + judge */}
        <aside className={styles.rightCol}>
          <div className={styles.teamSection}>
            <h2 className={styles.teamHeading}>Negative</h2>
            {negativeAgents.map((agent) => (
              <DebaterCard
                key={agent.id}
                agent={agent}
                status={agentStatus(agent.id)}
              />
            ))}
          </div>

          <div className={styles.judgeSection}>
            <JudgePanel sessionId={session.id} />
          </div>
        </aside>
      </div>

      {/* Timeline */}
      <footer className={styles.timeline}>
        {Array.from({ length: 13 }, (_, i) => i + 1).map((n) => {
          const done = n < session.currentRound;
          const active = n === session.currentRound;
          return (
            <div
              key={n}
              className={`${styles.timelineNode} ${done ? styles.done : ''} ${active ? styles.active : ''}`}
              title={`Round ${n}`}
            >
              {n}
            </div>
          );
        })}
      </footer>
    </div>
  );
}
