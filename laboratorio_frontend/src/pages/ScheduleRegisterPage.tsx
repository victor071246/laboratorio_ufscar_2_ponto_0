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

export default function ScheduleRegisterPage() {
  const [equipamentos, setEquipamentos] = useState<Equipamento[]>([]);
  const [status, setStatus] = useState<Status | null>(null);
  const [loading, setLoading] = useState(false);
  const [form, setForm] = useState({
    equipamento_uuid: '',
    data_inicio: '',
    data_fim: '',
    notificar_email: true,
    notificar_whatsapp: false,
    observacao: '',
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

  function updateField(field: keyof typeof form, value: string | boolean) {
    setForm((current) => ({ ...current, [field]: value }));
  }

  function toIsoDateTime(value: string) {
    return new Date(value).toISOString();
  }

  async function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    setLoading(true);
    setStatus(null);

    try {
      await api.post('/agendamentos', {
        equipamento_uuid: form.equipamento_uuid,
        data_inicio: toIsoDateTime(form.data_inicio),
        data_fim: toIsoDateTime(form.data_fim),
        notificar_email: form.notificar_email,
        notificar_whatsapp: form.notificar_whatsapp,
        observacao: form.observacao || null,
      });

      setStatus({
        type: 'success',
        message: 'Agendamento cadastrado com sucesso.',
      });
      setForm((current) => ({
        ...current,
        data_inicio: '',
        data_fim: '',
        observacao: '',
        notificar_email: true,
        notificar_whatsapp: false,
      }));
    } catch {
      setStatus({
        type: 'error',
        message: 'Nao foi possivel cadastrar o agendamento.',
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
          <p className={styles.eyebrow}>Uso do laboratorio</p>
          <h1>Cadastro de agendamentos</h1>
          <p>
            Reserve um equipamento para uma janela de uso e registre
            observacoes importantes para a rotina do laboratorio.
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

          <div className={styles.twoColumns}>
            <label>
              Inicio
              <input
                type="datetime-local"
                value={form.data_inicio}
                onChange={(event) =>
                  updateField('data_inicio', event.target.value)
                }
                required
              />
            </label>

            <label>
              Fim
              <input
                type="datetime-local"
                value={form.data_fim}
                onChange={(event) => updateField('data_fim', event.target.value)}
                required
              />
            </label>
          </div>

          <div className={styles.twoColumns}>
            <label>
              Email
              <select
                value={String(form.notificar_email)}
                onChange={(event) =>
                  updateField('notificar_email', event.target.value === 'true')
                }
              >
                <option value="true">Notificar</option>
                <option value="false">Nao notificar</option>
              </select>
            </label>

            <label>
              WhatsApp
              <select
                value={String(form.notificar_whatsapp)}
                onChange={(event) =>
                  updateField(
                    'notificar_whatsapp',
                    event.target.value === 'true',
                  )
                }
              >
                <option value="false">Nao notificar</option>
                <option value="true">Notificar</option>
              </select>
            </label>
          </div>

          <label>
            Observacao
            <textarea
              value={form.observacao}
              onChange={(event) => updateField('observacao', event.target.value)}
              rows={4}
            />
          </label>

          <button type="submit" disabled={loading || !form.equipamento_uuid}>
            {loading ? 'Cadastrando...' : 'Cadastrar agendamento'}
          </button>
        </form>
      </section>
      <Footer></Footer>
    </main>
  );
}
