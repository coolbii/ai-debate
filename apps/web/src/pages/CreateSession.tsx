import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useDebateStore } from '../store/debate';
import styles from './CreateSession.module.css';

const DEFAULT_MOTION = 'AI is capable of fully replacing human work';
const DEFAULT_DEFINITION =
  '"AI can fully replace human work" means that, under foreseeable technological and institutional conditions, AI and automation systems can perform the core functions of most economically transactable, standardisable, and process-oriented jobs, such that humans are no longer the primary routine executors of those roles.';

export function CreateSession() {
  const navigate = useNavigate();
  const createSession = useDebateStore((s) => s.createSession);
  const isLoading = useDebateStore((s) => s.isLoading);
  const error = useDebateStore((s) => s.error);

  const [motion, setMotion] = useState(DEFAULT_MOTION);
  const [definition, setDefinition] = useState(DEFAULT_DEFINITION);
  const [mode, setMode] = useState<'demo' | 'formal'>('demo');
  const [model, setModel] = useState('qwen3:4b');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const id = await createSession({ motion, definition, mode, model });
      navigate(`/session/${id}`);
    } catch {
      // error shown from store
    }
  };

  return (
    <div className={styles.page}>
      <div className={styles.card}>
        <h1 className={styles.title}>Agora Debate Theater</h1>
        <p className={styles.subtitle}>Set up a new Oregon-style 3v3 AI debate</p>

        <form onSubmit={handleSubmit} className={styles.form}>
          <label className={styles.label}>
            Motion
            <input
              className={styles.input}
              value={motion}
              onChange={(e) => setMotion(e.target.value)}
              required
              placeholder="The debate motion..."
            />
          </label>

          <label className={styles.label}>
            Definition
            <textarea
              className={styles.textarea}
              value={definition}
              onChange={(e) => setDefinition(e.target.value)}
              rows={5}
              required
              placeholder="Define the terms of the motion..."
            />
          </label>

          <div className={styles.row}>
            <label className={styles.label}>
              Mode
              <select
                className={styles.select}
                value={mode}
                onChange={(e) => setMode(e.target.value as 'demo' | 'formal')}
              >
                <option value="demo">Demo (30–60 s / turn)</option>
                <option value="formal">Formal (3 min / turn)</option>
              </select>
            </label>

            <label className={styles.label}>
              Model
              <input
                className={styles.input}
                value={model}
                onChange={(e) => setModel(e.target.value)}
                placeholder="qwen3:4b"
              />
            </label>
          </div>

          {error && <p className={styles.error}>{error}</p>}

          <button className={styles.button} type="submit" disabled={isLoading}>
            {isLoading ? 'Creating…' : 'Create Debate'}
          </button>
        </form>
      </div>
    </div>
  );
}
