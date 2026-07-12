import { describe, expect, it } from 'vitest';

import { normalizeParamListResponse } from './params';

describe('params api adapter', () => {
  it('normalizes params payload', () => {
    const result = normalizeParamListResponse({
      data: {
        list: [{ id: 1, name: 'Site name', key: 'site.name', value: 'Admin Console', desc: '' }],
        total: 1,
        page: 1,
        pageSize: 10,
      },
    });

    expect(result.list).toHaveLength(1);
    expect(result.total).toBe(1);
  });
});
