import type { DebateSide } from '@agora/shared-types';

export type PromptSection =
  | 'system_rules'
  | 'motion_and_definition'
  | 'team_stance'
  | 'debater_role'
  | 'public_summary'
  | 'turn_task'
  | 'json_contract';

export type DebaterRoleTemplate = {
  name: string;
  side: DebateSide;
  role: string;
  persona: string;
  objective: string;
  style: string;
  prohibitedBehavior: string[];
};

export type PromptBuildInput = {
  systemRules: string;
  motion: string;
  definition: string;
  side: DebateSide;
  roleTemplate: DebaterRoleTemplate;
  publicSummary: string;
  turnTask: string;
};

export type PromptBuildOutput = {
  sections: PromptSection[];
  prompt: string;
};

export const DEFAULT_PROMPT_SECTIONS: PromptSection[] = [
  'system_rules',
  'motion_and_definition',
  'team_stance',
  'debater_role',
  'public_summary',
  'turn_task',
  'json_contract',
];

export function buildPromptSpec(input: PromptBuildInput): PromptBuildOutput {
  const content = [
    `# System Rules\n${input.systemRules}`,
    `# Motion\n${input.motion}`,
    `# Definition\n${input.definition}`,
    `# Team Stance\n${input.side}`,
    `# Debater Role\n${input.roleTemplate.role}`,
    `# Persona\n${input.roleTemplate.persona}`,
    `# Objective\n${input.roleTemplate.objective}`,
    `# Style\n${input.roleTemplate.style}`,
    `# Public Summary\n${input.publicSummary}`,
    `# Turn Task\n${input.turnTask}`,
    '# JSON Contract\npublic_speech,key_claims,attack_targets,defense_targets,round_summary',
  ].join('\n\n');

  return {
    sections: DEFAULT_PROMPT_SECTIONS,
    prompt: content,
  };
}
