import { Navigate, Route, Routes } from 'react-router-dom';
import { CreateSession } from '../pages/CreateSession';
import { DebateStage } from '../pages/DebateStage';
import { Replay } from '../pages/Replay';

export function App() {
  return (
    <Routes>
      <Route path="/" element={<Navigate to="/create" replace />} />
      <Route path="/create" element={<CreateSession />} />
      <Route path="/session/:id" element={<DebateStage />} />
      <Route path="/session/:id/replay" element={<Replay />} />
    </Routes>
  );
}

export default App;
