import { OREGON_MVP_TURN_PLAN } from '@agora/debate-protocol';
import { DebaterCard } from '@agora/ui';
import type { DebaterAgent } from '@agora/shared-types';
import styles from './app.module.css';

const affirmativeTeam: Pick<DebaterAgent, 'displayName' | 'role' | 'model'>[] = [
  { displayName: 'Affirmative First', role: 'first', model: 'qwen3:4b' },
  { displayName: 'Affirmative Second', role: 'second', model: 'qwen3:4b' },
  { displayName: 'Affirmative Third', role: 'third', model: 'qwen3:4b' },
];

const negativeTeam: Pick<DebaterAgent, 'displayName' | 'role' | 'model'>[] = [
  { displayName: 'Negative First', role: 'first', model: 'qwen3:4b' },
  { displayName: 'Negative Second', role: 'second', model: 'qwen3:4b' },
  { displayName: 'Negative Third', role: 'third', model: 'qwen3:4b' },
];

export function App() {
  const currentTurn = OREGON_MVP_TURN_PLAN[0];

  return (
    <main className={styles['layout']}>
      <header className={styles['header']}>
        <h1>Agora Debate Theater</h1>
        <p>Oregon-style 3v3 AI debate MVP workspace</p>
      </header>

      <section className={styles['board']}>
        <aside className={styles['team']}>
          <h2>Affirmative</h2>
          {affirmativeTeam.map((agent) => (
            <DebaterCard key={agent.displayName} agent={agent} status="standby" />
          ))}
        </aside>

        <section className={styles['stage']}>
          <h2>Debate Stage</h2>
          <p>
            <strong>Motion:</strong> Can AI fully replace human work?
          </p>
          <p>
            <strong>Current phase:</strong> {currentTurn.phase}
          </p>
          <p>
            <strong>Planned turns:</strong> {OREGON_MVP_TURN_PLAN.length}
          </p>
        </section>

        <aside className={styles['team']}>
          <h2>Negative</h2>
          {negativeTeam.map((agent) => (
            <DebaterCard key={agent.displayName} agent={agent} status="standby" />
          ))}
        </aside>
      </section>

      <div className={styles['timeline']}>
        <h3>MVP turn sequence preview</h3>
        <ol>
          {OREGON_MVP_TURN_PLAN.map((turn) => (
            <li key={turn.index}>
              {turn.index}. {turn.phase}
            </li>
          ))}
        </ol>
      </div>
    </main>
  );
}

export default App;
