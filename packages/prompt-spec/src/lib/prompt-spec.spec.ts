import { buildPromptSpec } from './prompt-spec';

describe('prompt-spec', () => {
  it('should build prompt blocks in the expected order', () => {
    const output = buildPromptSpec({
      systemRules: 'Stay in side and follow phase rules.',
      motion: 'AI can fully replace human work',
      definition: 'Definition is locked at session creation.',
      side: 'affirmative',
      roleTemplate: {
        name: 'Affirmative First',
        side: 'affirmative',
        role: 'first',
        persona: 'Structured and assertive',
        objective: 'Establish framework and burden',
        style: 'concise',
        prohibitedBehavior: ['switch_side'],
      },
      publicSummary: 'No prior turns',
      turnTask: 'Deliver opening constructive speech',
    });

    expect(output.sections[0]).toEqual('system_rules');
    expect(output.prompt).toContain('# JSON Contract');
  });
});
