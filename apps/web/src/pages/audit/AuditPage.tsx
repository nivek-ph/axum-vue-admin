import { useQuery } from '@tanstack/react-query';
import { Search } from 'lucide-react';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';

import { fetchAuditEvent, fetchAuditEvents, type AuditEventRecord } from '@/api/audit';
import { Button } from '@/components/ui/Button';
import { Modal } from '@/components/ui/Modal';

export function AuditPage() {
  const { t } = useTranslation();
  const [page, setPage] = useState(1);
  const [draft, setDraft] = useState({ actor: '', action: '', resourceType: '', result: '' });
  const [filters, setFilters] = useState(draft);
  const [selectedId, setSelectedId] = useState<number | null>(null);
  const list = useQuery({ queryKey: ['audit-events', page, filters], queryFn: () => fetchAuditEvents({ page, pageSize: 10, ...filters }) });
  const detail = useQuery({ queryKey: ['audit-event', selectedId], queryFn: () => fetchAuditEvent(selectedId as number), enabled: selectedId !== null });
  const pages = Math.max(1, Math.ceil((list.data?.total ?? 0) / 10));
  const selected = detail.data;

  return (
    <div className="page-stack">
      <header className="page-header">
        <div>
          <p className="eyebrow">{t('Operational trace')}</p>
          <h1>{t('Audit events')}</h1>
          <p>{t('Inspect administrative actions, outcomes, and structured changes.')}</p>
        </div>
      </header>
      <section className="panel">
        <div className="toolbar audit-filters">
          <input aria-label="Filter by actor" onChange={(event) => setDraft((current) => ({ ...current, actor: event.target.value }))} placeholder={t('Actor')} value={draft.actor} />
          <input aria-label="Filter by action" onChange={(event) => setDraft((current) => ({ ...current, action: event.target.value }))} placeholder={t('Action')} value={draft.action} />
          <input aria-label="Filter by resource" onChange={(event) => setDraft((current) => ({ ...current, resourceType: event.target.value }))} placeholder={t('Resource')} value={draft.resourceType} />
          <select aria-label="Filter by result" onChange={(event) => setDraft((current) => ({ ...current, result: event.target.value }))} value={draft.result}>
            <option value="">{t('All results')}</option>
            <option value="succeeded">{t('Succeeded')}</option>
            <option value="denied">{t('Denied')}</option>
            <option value="failed">{t('Failed')}</option>
          </select>
          <Button
            onClick={() => {
              setPage(1);
              setFilters(draft);
            }}
          >
            <Search size={16} />
            {t('Search')}
          </Button>
        </div>
        <div className="table-wrap">
          <table>
            <thead>
              <tr>
                <th>{t('Time')}</th>
                <th>{t('Actor')}</th>
                <th>{t('Action')}</th>
                <th>{t('Resource')}</th>
                <th>{t('Result')}</th>
                <th>{t('Source IP')}</th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              {list.data?.list.map((item) => (
                <AuditRow item={item} key={item.id} onOpen={() => setSelectedId(item.id)} />
              ))}
              {!list.isLoading && !list.data?.list.length && (
                <tr>
                  <td className="empty-cell" colSpan={7}>
                    {t('No audit events')}
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
        <div className="pagination">
          <Button disabled={page <= 1} onClick={() => setPage((current) => current - 1)}>
            {t('Previous')}
          </Button>
          <span>
            {t('Page')} {page} / {pages}
          </span>
          <Button disabled={page >= pages} onClick={() => setPage((current) => current + 1)}>
            {t('Next')}
          </Button>
        </div>
      </section>
      <Modal
        onOpenChange={(open) => {
          if (!open) setSelectedId(null);
        }}
        open={selectedId !== null}
        title={t('Audit event detail')}
      >
        {selected ? (
          <div className="audit-detail">
            <dl>
              <dt>{t('Actor')}</dt>
              <dd>{selected.actorLabel}</dd>
              <dt>{t('Action')}</dt>
              <dd>{selected.action}</dd>
              <dt>{t('Resource')}</dt>
              <dd>
                {selected.resourceType} / {selected.resourceId || '—'}
              </dd>
              <dt>{t('Result')}</dt>
              <dd>{t(selected.result === 'succeeded' ? 'Succeeded' : selected.result === 'denied' ? 'Denied' : 'Failed')}</dd>
              <dt>{t('Reason code')}</dt>
              <dd>{selected.reasonCode || '—'}</dd>
              <dt>{t('Source IP')}</dt>
              <dd>{selected.sourceIp}</dd>
              <dt>{t('User agent')}</dt>
              <dd>{selected.userAgent}</dd>
              <dt>{t('Created at')}</dt>
              <dd>{selected.createdAt}</dd>
            </dl>
            <h3>{t('Changes')}</h3>
            <pre>{JSON.stringify(selected.changes, null, 2)}</pre>
          </div>
        ) : (
          <p>{t('Loading event…')}</p>
        )}
      </Modal>
    </div>
  );
}

function AuditRow({ item, onOpen }: { item: AuditEventRecord; onOpen: () => void }) {
  const { t } = useTranslation();
  return (
    <tr>
      <td>{item.createdAt}</td>
      <td>{item.actorLabel}</td>
      <td>
        <code>{item.action}</code>
      </td>
      <td>
        {item.resourceType}
        <small>{item.resourceId || '—'}</small>
      </td>
      <td>
        <span className={`status ${item.result}`}>{t(item.result === 'succeeded' ? 'Succeeded' : item.result === 'denied' ? 'Denied' : 'Failed')}</span>
      </td>
      <td>{item.sourceIp}</td>
      <td>
        <Button onClick={onOpen} variant="ghost">
          {t('View detail')}
        </Button>
      </td>
    </tr>
  );
}
