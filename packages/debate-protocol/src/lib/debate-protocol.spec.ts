import {
  getPlannedTurn,
  isCrossExamination,
  OREGON_MVP_TURN_PLAN,
} from './debate-protocol';

describe('debate-protocol', () => {
  it('should include 13 planned phases for MVP', () => {
    expect(OREGON_MVP_TURN_PLAN).toHaveLength(13);
  });

  it('should map cross-examination rounds', () => {
    const round2 = getPlannedTurn(2);

    expect(round2).toBeDefined();
    if (!round2) {
      throw new Error('Expected round 2 to exist');
    }

    expect(isCrossExamination(round2)).toBe(true);
  });
});
