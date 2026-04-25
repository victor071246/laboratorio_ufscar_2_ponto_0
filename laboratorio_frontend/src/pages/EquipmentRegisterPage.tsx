import { useState } from 'react';
import { Link } from 'react-router-dom';
import api from './../services/api';
import styles from './RegisterPage.module.css';

type Status = {
  type: 'success' | 'error';
  message: string;
};

export default function EquipmentRegisterPage() {
  const [status, setStatus] = useState<Status | null>(null);
  const [loading, setLoading] = useState(false);
  const [form, setForm] = useState({
    nome: '',
    descricao: '',
    estado: 'disponivel',
    data_aquisicao: '',
    peso_kg: '',
    largura_cm: '',
    altura_cm: '',
    profundidade_cm: '',
  });

  function updateField(field: keyof typeof form, value: string) {
    setForm((current) => ({ ...current, [field]: value }));
  }

  function optionalNumber(value: string) {
    return value === '' ? null : Number(value);
  }

  async function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    setLoading(true);
    setStatus(null);

    try {
      await api.post('/equipamentos', {
        nome: form.nome,
        descricao: form.descricao || null,
        estado: form.estado,
        data_aquisicao: form.data_aquisicao || null,
        peso_kg: optionalNumber(form.peso_kg),
        largura_cm: optionalNumber(form.largura_cm),
        altura_cm: optionalNumber(form.altura_cm),
        profundidade_cm: optionalNumber(form.profundidade_cm),
      });
      setStatus({ type: 'success', message: 'Equipamento cadastrado com sucesso.' });
      setForm({
        nome: '',
        descricao: '',
        estado: 'disponivel',
        data_aquisicao: '',
        peso_kg: '',
        largura_cm: '',
        altura_cm: '',
        profundidade_cm: '',
      });
    } catch {
      setStatus({ type: 'error', message: 'Nao foi possivel cadastrar o equipamento.' });
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className={styles.page}>
      <header className={styles.header}>
        <Link to="/">Inicio</Link>
        <Link to="/cadastro/usuarios">Cadastrar usuario</Link>
      </header>

      <section className={styles.shell}>
        <aside className={styles.intro}>
          <p className={styles.eyebrow}>Patrimonio analitico</p>
          <h1>Cadastro de equipamentos</h1>
          <p>
            Inclua instrumentos do laboratorio com dimensoes, estado de uso e dados
            de aquisicao para apoiar controle, manutencao e futuros agendamentos.
          </p>
        </aside>

        <form className={styles.form} onSubmit={handleSubmit}>
          {status && <p className={styles[status.type]}>{status.message}</p>}

          <label>
            Nome do equipamento
            <input
              value={form.nome}
              onChange={(event) => updateField('nome', event.target.value)}
              required
            />
          </label>

          <label>
            Descricao
            <textarea
              value={form.descricao}
              onChange={(event) => updateField('descricao', event.target.value)}
              rows={4}
            />
          </label>

          <div className={styles.twoColumns}>
            <label>
              Estado
              <select value={form.estado} onChange={(event) => updateField('estado', event.target.value)}>
                <option value="disponivel">Disponivel</option>
                <option value="em_manutencao">Em manutencao</option>
                <option value="quebrado">Quebrado</option>
                <option value="reservado">Reservado</option>
                <option value="desativado">Desativado</option>
              </select>
            </label>

            <label>
              Data de aquisicao
              <input
                type="date"
                value={form.data_aquisicao}
                onChange={(event) => updateField('data_aquisicao', event.target.value)}
              />
            </label>
          </div>

          <div className={styles.fourColumns}>
            <label>
              Peso (kg)
              <input
                type="number"
                step="0.01"
                value={form.peso_kg}
                onChange={(event) => updateField('peso_kg', event.target.value)}
              />
            </label>
            <label>
              Largura (cm)
              <input
                type="number"
                step="0.01"
                value={form.largura_cm}
                onChange={(event) => updateField('largura_cm', event.target.value)}
              />
            </label>
            <label>
              Altura (cm)
              <input
                type="number"
                step="0.01"
                value={form.altura_cm}
                onChange={(event) => updateField('altura_cm', event.target.value)}
              />
            </label>
            <label>
              Profundidade (cm)
              <input
                type="number"
                step="0.01"
                value={form.profundidade_cm}
                onChange={(event) => updateField('profundidade_cm', event.target.value)}
              />
            </label>
          </div>

          <button type="submit" disabled={loading}>
            {loading ? 'Cadastrando...' : 'Cadastrar equipamento'}
          </button>
        </form>
      </section>
    </main>
  );
}
