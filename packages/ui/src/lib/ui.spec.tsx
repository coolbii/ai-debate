import { render } from '@testing-library/react';
import { screen } from '@testing-library/dom';

import DebaterCard from './ui';

describe('DebaterCard', () => {
  it('should render successfully', () => {
    const { baseElement } = render(
      <DebaterCard
        agent={{
          displayName: 'Affirmative First',
          role: 'first',
          model: 'qwen3:4b',
        }}
        status="standby"
      />,
    );
    expect(baseElement).toBeTruthy();
  });

  it('should render role and model', () => {
    render(
      <DebaterCard
        agent={{
          displayName: 'Negative Second',
          role: 'second',
          model: 'qwen3:4b',
        }}
        status="speaking"
      />,
    );

    expect(screen.getByText(/role: second/i)).toBeTruthy();
    expect(screen.getByText(/model: qwen3:4b/i)).toBeTruthy();
  });
});
