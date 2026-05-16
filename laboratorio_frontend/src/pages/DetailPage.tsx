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
import Header from '../components/Header';
import Footer from '../components/Footer';

export function CardDetailPage({ tabela }: { tabela: EntityKey }) {
  const { id } = useParams();
  const config = entities[tabela];
  const [dados, setDados] = useState<Record<string, unknown> | null>(null);
  const [editMode, setEditMode] = useState(false);
  const [rascunho, setRascunho] = useState<Record<string, unknown>>({});

  function entrarEdicao() {
    setRascunho({ ...dados });
    setEditMode(true);
  }

  function cancelarEdicao() {
    setEditMode(false);
    setRascunho({});
  }

  async function salvar() {
    await api.put(`${config.endpoint}/${id}`, rascunho);
    const res = await api.get(`${config.endpoint}/${id}`);
    setDados(res.data.data);
    setEditMode(false);
  }

  async function cancelarAgendamento() {
    await api.post(`/agendamentos/${id}/cancelar`);
    const res = await api.get(`${config.endpoint}/${id}`);
    setDados(res.data.data);
  }

  useEffect(() => {
    api.get(`${config.endpoint}/${id}`).then((r) => setDados(r.data.data));
  }, [id]);

  if (!dados) return <span>Carregando</span>;

  return (
    <div className={styles.page}>
      <Header></Header>
      <div className={styles.content}>
        <div className={styles.card}>
          {Object.entries(dados)
            .filter(([campo]) => !config.hiddenFields?.includes(campo))
            .map(([campo, valor]) => (
              <div key={campo} className={styles.field}>
                <span className={styles.label}>{formatColumnName(campo)}</span>
                {editMode && config.editableFields.includes(campo) ? (
                  <input
                    className={styles.input}
                    value={String(rascunho[campo] ?? '')}
                    onChange={(e) =>
                      setRascunho((prev) => ({
                        ...prev,
                        [campo]: e.target.value,
                      }))
                    }
                  ></input>
                ) : (
                  <span className={styles.value}>{formatCellValue(valor)}</span>
                )}
              </div>
            ))}

          <div className={styles.actions}>
            {tabela === 'equipamentos' &&
              (editMode ? (
                <>
                  <button onClick={salvar}>Salvar</button>
                  <button onClick={cancelarEdicao}>Cancelar</button>
                </>
              ) : (
                <button onClick={entrarEdicao}>Editar</button>
              ))}
            {tabela === 'agendamentos' && dados['status'] !== 'cancelado' && (
              <button onClick={cancelarAgendamento}>
                Cancelar agendamento
              </button>
            )}
          </div>
        </div>
      </div>
      <Footer></Footer>
    </div>
  );
}
