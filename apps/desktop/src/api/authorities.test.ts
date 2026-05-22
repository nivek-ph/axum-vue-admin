import { describe, expect, it } from 'vitest';

import { normalizeAuthorityListResponse } from './authorities';

describe('authority api adapter', () => {
  it('normalizes backend authority tree payload', () => {
    const result = normalizeAuthorityListResponse({
      data: [
        {
          authorityId: 888,
          authorityName: '超级管理员',
          children: [{ authorityId: 999, authorityName: '运营' }],
        },
      ],
    });

    expect(result).toHaveLength(1);
    expect(result[0].children).toHaveLength(1);
  });
});
