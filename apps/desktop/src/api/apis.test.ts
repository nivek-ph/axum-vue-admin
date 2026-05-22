import { describe, expect, it } from 'vitest';

import { normalizeApiListResponse, normalizeApiRoleSelection } from './apis';

describe('api registry adapter', () => {
  it('normalizes paginated api payload', () => {
    const result = normalizeApiListResponse({
      data: {
        list: [{ ID: 1, path: '/api/users', method: 'GET', apiGroup: 'user', description: 'Get user list' }],
        total: 1,
        page: 1,
        pageSize: 10,
      },
    });

    expect(result.list).toHaveLength(1);
    expect(result.total).toBe(1);
  });

  it('normalizes assigned api role ids', () => {
    const result = normalizeApiRoleSelection({
      data: {
        authorityIds: [888],
      },
    });

    expect(result.authorityIds).toEqual([888]);
  });
});
