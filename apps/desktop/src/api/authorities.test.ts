import { describe, expect, it } from 'vitest';

import { normalizeAuthorityListResponse } from './authorities';

describe('authority api adapter', () => {
  it('normalizes backend authority tree payload', () => {
    const result = normalizeAuthorityListResponse({
      data: [
        {
          authorityId: 888,
          authorityName: 'Super Admin',
          children: [{ authorityId: 999, authorityName: 'Operator' }],
        },
      ],
    });

    expect(result).toHaveLength(1);
    expect(result[0].children).toHaveLength(1);
  });
});
