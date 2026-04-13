import styles from './RoundStatusBanner.module.css';

interface Props {
  round: number;
  phase: string;
  status: string;
  isRunningTurn: boolean;
  speakerName?: string;
}

const PHASE_LABELS: Record<string, string> = {
  'affirmative-first-constructive': 'Affirmative First — Constructive Speech',
  'negative-second-cross-affirmative-first': 'Negative Second cross-examines Affirmative First',
  'negative-first-constructive': 'Negative First — Constructive Speech',
  'affirmative-third-cross-negative-first': 'Affirmative Third cross-examines Negative First',
  'affirmative-second-constructive': 'Affirmative Second — Constructive Speech',
  'negative-third-cross-affirmative-second': 'Negative Third cross-examines Affirmative Second',
  'negative-second-constructive': 'Negative Second — Constructive Speech',
  'affirmative-first-cross-negative-second': 'Affirmative First cross-examines Negative Second',
  'affirmative-third-constructive': 'Affirmative Third — Constructive Speech',
  'negative-first-cross-affirmative-third': 'Negative First cross-examines Affirmative Third',
  'negative-third-constructive': 'Negative Third — Constructive Speech',
  'affirmative-second-cross-negative-third': 'Affirmative Second cross-examines Negative Third',
  closing: 'Closing Statements',
  finished: 'Debate Finished',
  '': 'Not started',
};

export function RoundStatusBanner({ round, phase, status, isRunningTurn, speakerName }: Props) {
  const label = PHASE_LABELS[phase] ?? phase;

  return (
    <div className={`${styles.banner} ${styles[status] ?? ''}`}>
      <div className={styles.left}>
        <span className={styles.round}>Round {round} / 13</span>
        <span className={styles.phase}>{label}</span>
        {speakerName && <span className={styles.speaker}>Speaking: {speakerName}</span>}
      </div>
      <div className={styles.right}>
        {isRunningTurn && <span className={styles.generating}>Generating…</span>}
        {status === 'paused' && <span className={styles.paused}>PAUSED</span>}
        {status === 'finished' && <span className={styles.done}>DONE</span>}
      </div>
    </div>
  );
}
