import { useState } from 'react';
import { useDebateStore } from '../store/debate';
import styles from './JudgePanel.module.css';

interface Props {
  sessionId: string;
}

export function JudgePanel({ sessionId: _sessionId }: Props) {
  const session = useDebateStore((s) => s.session);
  const decision = useDebateStore((s) => s.decision);
  const isRunningTurn = useDebateStore((s) => s.isRunningTurn);
  const error = useDebateStore((s) => s.error);

  const startSession = useDebateStore((s) => s.startSession);
  const pauseSession = useDebateStore((s) => s.pauseSession);
  const resumeSession = useDebateStore((s) => s.resumeSession);
  const nextTurn = useDebateStore((s) => s.nextTurn);
  const autoRun = useDebateStore((s) => s.autoRun);
  const stopAutoRun = useDebateStore((s) => s.stopAutoRun);
  const addJudgeNote = useDebateStore((s) => s.addJudgeNote);
  const declareWinner = useDebateStore((s) => s.declareWinner);
  const clearError = useDebateStore((s) => s.clearError);

  const [note, setNote] = useState('');
  const [winnerSide, setWinnerSide] = useState<'affirmative' | 'negative'>('affirmative');
  const [reasoning, setReasoning] = useState('');
  const [isAutoRunning, setIsAutoRunning] = useState(false);

  if (!session) return null;

  const status = session.status;
  const isDraft = status === 'draft';
  const isRunning = status === 'running';
  const isPaused = status === 'paused';
  const isFinished = status === 'finished';

  const handleAutoRun = () => {
    setIsAutoRunning(true);
    autoRun(500); // 500ms between turns in auto mode
  };

  const handleStopAutoRun = () => {
    setIsAutoRunning(false);
    stopAutoRun();
  };

  const handleNote = async () => {
    if (!note.trim()) return;
    await addJudgeNote(note.trim());
    setNote('');
  };

  const handleDeclare = async () => {
    if (!reasoning.trim()) return;
    await declareWinner(winnerSide, reasoning.trim());
  };

  return (
    <div className={styles.panel}>
      <h3 className={styles.heading}>Judge Controls</h3>

      {error && (
        <div className={styles.error}>
          {error}
          <button className={styles.clearBtn} onClick={clearError}>×</button>
        </div>
      )}

      {/* Debate flow controls */}
      <div className={styles.controls}>
        {isDraft && (
          <button className={`${styles.btn} ${styles.primary}`} onClick={startSession}>
            Start Debate
          </button>
        )}
        {isRunning && (
          <>
            <button
              className={`${styles.btn} ${styles.secondary}`}
              onClick={nextTurn}
              disabled={isRunningTurn}
            >
              {isRunningTurn ? 'Running…' : 'Next Turn'}
            </button>
            {!isAutoRunning ? (
              <button className={`${styles.btn} ${styles.ghost}`} onClick={handleAutoRun}>
                Auto Run
              </button>
            ) : (
              <button className={`${styles.btn} ${styles.warning}`} onClick={handleStopAutoRun}>
                Stop Auto
              </button>
            )}
            <button className={`${styles.btn} ${styles.ghost}`} onClick={pauseSession}>
              Pause
            </button>
          </>
        )}
        {isPaused && (
          <button className={`${styles.btn} ${styles.primary}`} onClick={resumeSession}>
            Resume
          </button>
        )}
      </div>

      {/* Judge note */}
      {!isFinished && !isDraft && (
        <div className={styles.section}>
          <label className={styles.label}>Add Note</label>
          <div className={styles.noteRow}>
            <input
              className={styles.input}
              value={note}
              onChange={(e) => setNote(e.target.value)}
              placeholder="Your observation…"
              onKeyDown={(e) => e.key === 'Enter' && handleNote()}
            />
            <button className={`${styles.btn} ${styles.ghost}`} onClick={handleNote}>
              Add
            </button>
          </div>
        </div>
      )}

      {/* Declare winner */}
      {(isFinished || isPaused || isRunning) && !decision && (
        <div className={styles.section}>
          <label className={styles.label}>Declare Winner</label>
          <select
            className={styles.select}
            value={winnerSide}
            onChange={(e) => setWinnerSide(e.target.value as 'affirmative' | 'negative')}
          >
            <option value="affirmative">Affirmative</option>
            <option value="negative">Negative</option>
          </select>
          <textarea
            className={styles.textarea}
            value={reasoning}
            onChange={(e) => setReasoning(e.target.value)}
            placeholder="Reasoning for the decision…"
            rows={3}
          />
          <button className={`${styles.btn} ${styles.primary}`} onClick={handleDeclare}>
            Declare
          </button>
        </div>
      )}

      {/* Decision display */}
      {decision && (
        <div className={styles.decision}>
          <div className={styles.winner}>
            Winner: <strong>{decision.winner.toUpperCase()}</strong>
          </div>
          <p className={styles.reasoning}>{decision.reasoning}</p>
        </div>
      )}
    </div>
  );
}
