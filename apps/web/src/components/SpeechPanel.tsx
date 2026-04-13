import { useEffect, useRef } from 'react';
import { DebateEvent, DebaterAgent } from '../api/client';
import styles from './SpeechPanel.module.css';

interface Props {
  events: DebateEvent[];
  agents: DebaterAgent[];
}

function agentName(speakerId: string, agents: DebaterAgent[]): string {
  if (speakerId === 'system') return 'System';
  if (speakerId === 'judge') return 'Judge';
  return agents.find((a) => a.id === speakerId)?.displayName ?? speakerId;
}

function kindLabel(kind: string): string {
  switch (kind) {
    case 'speech': return 'Speech';
    case 'question': return 'Cross-Exam';
    case 'answer': return 'Answer';
    case 'judge_note': return 'Judge Note';
    default: return '';
  }
}

function agentSide(speakerId: string, agents: DebaterAgent[]): string {
  return agents.find((a) => a.id === speakerId)?.side ?? 'system';
}

export function SpeechPanel({ events, agents }: Props) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [events.length]);

  const speechEvents = events.filter((e) => e.kind !== 'system');

  return (
    <div className={styles.panel}>
      <h3 className={styles.heading}>Transcript</h3>
      <div className={styles.list}>
        {speechEvents.length === 0 && (
          <p className={styles.empty}>Debate has not started yet.</p>
        )}
        {speechEvents.map((event) => {
          const side = agentSide(event.speakerId, agents);
          return (
            <div key={event.id} className={`${styles.entry} ${styles[side]}`}>
              <div className={styles.meta}>
                <span className={styles.name}>{agentName(event.speakerId, agents)}</span>
                <span className={styles.kind}>{kindLabel(event.kind)}</span>
                <span className={styles.phase}>{event.phase}</span>
              </div>
              <p className={styles.content}>{event.content}</p>
              {event.meta && (event.meta['key_claims'] as string[] | undefined)?.length ? (
                <ul className={styles.claims}>
                  {(event.meta['key_claims'] as string[]).map((c, i) => (
                    <li key={i}>{c}</li>
                  ))}
                </ul>
              ) : null}
            </div>
          );
        })}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}
