import { describe, expect, it } from 'vitest';

import { normalizeUserListResponse } from './users';

describe('user api adapter', () => {
  it('normalizes backend list payload', () => {
    const result = normalizeUserListResponse({
      data: {
        list: [{ ID: 1, userName: 'admin' }],
        total: 1,
        page: 1,
        pageSize: 10,
      },
    });

    expect(result.list).toHaveLength(1);
    expect(result.total).toBe(1);
  });
});
