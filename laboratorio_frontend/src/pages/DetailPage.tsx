import { useParams } from 'react-router-dom';
import { useEffect, useState } from 'react';
import type { EntityKey } from '../config/entities';
import {
  entities,
  formatColumnName,
  formatCellValue,
} from '../config/entities';
import api from '../services/api';
import styles from './DetailPage.module.css';

export function CardDetailPage({ tabela }: { tabela: EntityKey }) {
  const { id } = useParams();
  const config = entities[tabela];
  const [dados, setDados] = useState<Record<string, unknown> | null>(null);

  useEffect(() => {
    api.get(`${config.endpoint}/${id}`).then((r) => setDados(r.data));
  }, [id]);

  if (!dados) return <span>Carregando</span>;

  return (
    <div className={styles.page}>
      <div className={styles.content}>
        <div className={styles.card}>
          {Object.entries(dados)
            .filter(([campo]) => !config.hiddenFields?.includes(campo))
            .map(([campo, valor]) => (
              <div key={campo} className={styles.field}>
                <span className={styles.label}>{formatColumnName(campo)}</span>
                <span className={styles.value}>{formatCellValue(valor)}</span>
              </div>
            ))}
        </div>
      </div>
    </div>
  );
}
