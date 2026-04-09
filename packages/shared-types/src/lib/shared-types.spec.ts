import type { DebateSession } from './shared-types';

describe('shared-types', () => {
  it('should expose the session contract', () => {
    const session: DebateSession = {
      id: 'session-1',
      motion: 'Can AI fully replace human work?',
      definition: 'Definition lock test',
      format: 'oregon-3v3',
      mode: 'demo',
      status: 'draft',
      currentRound: 0,
      currentPhase: 'setup',
      affirmativeTeamId: 'team-a',
      negativeTeamId: 'team-b',
      judgeId: 'judge-1',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    expect(session.format).toEqual('oregon-3v3');
  });
});
