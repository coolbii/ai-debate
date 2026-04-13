import styles from './ui.module.css';

export type DebaterCardProps = {
  agent: { displayName: string; role?: string; seat?: string; model: string };
  status: 'standby' | 'speaking' | 'completed';
};

export function DebaterCard({ agent, status }: DebaterCardProps) {
  const statusClassName = {
    standby: styles['status-standby'],
    speaking: styles['status-speaking'],
    completed: styles['status-completed'],
  }[status];

  return (
    <div className={styles['container']}>
      <div className={styles['header']}>
        <strong>{agent.displayName}</strong>
        <span className={`${styles['status']} ${statusClassName}`}>{status}</span>
      </div>
      <div className={styles['meta']}>
        <span>role: {agent.seat ?? agent.role}</span>
        <span>model: {agent.model}</span>
      </div>
    </div>
  );
}

export default DebaterCard;
