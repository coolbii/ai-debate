import styles from './DefinitionPanel.module.css';

interface Props {
  motion: string;
  definition: string;
  mode: string;
}

export function DefinitionPanel({ motion, definition, mode }: Props) {
  return (
    <div className={styles.panel}>
      <div className={styles.mode}>{mode.toUpperCase()} MODE</div>
      <h2 className={styles.motion}>{motion}</h2>
      <p className={styles.definition}>{definition}</p>
    </div>
  );
}
