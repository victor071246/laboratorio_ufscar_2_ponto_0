import { useEffect, useState } from 'react';
import api from './../services/api';
import styles from './RegisterPage.module.css';
import Header from '../components/Header';
import Footer from '../components/Footer';

type Status = {
  type: 'success' | 'error';
  message: string;
};

type Equipamento = {
  uuid: string;
  nome: string;
  estado?: string;
};

export default function OccurrenceRegisterPage() {
  const [equipamentos, setEquipamentos] = useState<Equipamento[]>([]);
  const [status, setStatus] = useState<Status | null>(null);
  const [loading, setLoading] = useState(false);
  const [form, setForm] = useState({
    equipamento_uuid: '',
    tipo: 'manutencao',
    descricao: '',
  });

  useEffect(() => {
    api
      .get('/equipamentos')
      .then((res) => {
        const lista = res.data.data ?? [];
        setEquipamentos(lista);
        setForm((current) => ({
          ...current,
          equipamento_uuid: current.equipamento_uuid || lista[0]?.uuid || '',
        }));
      })
      .catch(() =>
        setStatus({
          type: 'error',
          message: 'Nao foi possivel carregar os equipamentos.',
        }),
      );
  }, []);

  function updateField(field: keyof typeof form, value: string) {
    setForm((current) => ({ ...current, [field]: value }));
  }

  async function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    setLoading(true);
    setStatus(null);

    try {
      await api.post('/ocorrencias', {
        equipamento_uuid: form.equipamento_uuid,
        tipo: form.tipo,
        descricao: form.descricao,
      });

      setStatus({
        type: 'success',
        message: 'Ocorrencia cadastrada com sucesso.',
      });
      setForm((current) => ({
        ...current,
        tipo: 'manutencao',
        descricao: '',
      }));
    } catch {
      setStatus({
        type: 'error',
        message: 'Nao foi possivel cadastrar a ocorrencia.',
      });
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className={styles.page}>
      <Header></Header>
      <section className={styles.shell}>
        <aside className={styles.intro}>
          <p className={styles.eyebrow}>Registro operacional</p>
          <h1>Cadastro de ocorrencias</h1>
          <p>
            Registre manutencoes, defeitos, acidentes ou observacoes relevantes
            associadas aos equipamentos do laboratorio.
          </p>
        </aside>

        <form className={styles.form} onSubmit={handleSubmit}>
          {status && <p className={styles[status.type]}>{status.message}</p>}

          <label>
            Equipamento
            <select
              value={form.equipamento_uuid}
              onChange={(event) =>
                updateField('equipamento_uuid', event.target.value)
              }
              required
            >
              {equipamentos.map((equipamento) => (
                <option key={equipamento.uuid} value={equipamento.uuid}>
                  {equipamento.nome}
                  {equipamento.estado ? ` - ${equipamento.estado}` : ''}
                </option>
              ))}
            </select>
          </label>

          <label>
            Tipo
            <select
              value={form.tipo}
              onChange={(event) => updateField('tipo', event.target.value)}
            >
              <option value="manutencao">Manutencao</option>
              <option value="defeito">Defeito</option>
              <option value="acidente">Acidente</option>
              <option value="outro">Outro</option>
            </select>
          </label>

          <label>
            Descricao
            <textarea
              value={form.descricao}
              onChange={(event) => updateField('descricao', event.target.value)}
              rows={5}
              required
            />
          </label>

          <button type="submit" disabled={loading || !form.equipamento_uuid}>
            {loading ? 'Cadastrando...' : 'Cadastrar ocorrencia'}
          </button>
        </form>
      </section>
      <Footer></Footer>
    </main>
  );
}
