import { useState } from 'react';
import { Link } from 'react-router-dom';
import api from './../services/api';
import styles from './RegisterPage.module.css';

type Status = {
  type: 'success' | 'error';
  message: string;
};

export default function UserRegisterPage() {
  const [nome, setNome] = useState('');
  const [email, setEmail] = useState('');
  const [senha, setSenha] = useState('');
  const [telefone, setTelefone] = useState('');
  const [papel, setPapel] = useState('aluno');
  const [status, setStatus] = useState<Status | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    setLoading(true);
    setStatus(null);

    try {
      await api.post('/auth/registrar', {
        nome,
        email,
        senha,
        papel,
        telefone: telefone || null,
      });
      setStatus({ type: 'success', message: 'Usuario cadastrado com sucesso.' });
      setNome('');
      setEmail('');
      setSenha('');
      setTelefone('');
      setPapel('aluno');
    } catch {
      setStatus({ type: 'error', message: 'Nao foi possivel cadastrar o usuario.' });
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className={styles.page}>
      <header className={styles.header}>
        <Link to="/">Inicio</Link>
        <Link to="/cadastro/equipamentos">Cadastrar equipamento</Link>
      </header>

      <section className={styles.shell}>
        <aside className={styles.intro}>
          <p className={styles.eyebrow}>Pessoas do laboratorio</p>
          <h1>Cadastro de usuarios</h1>
          <p>
            Registre alunos, supervisores e administradores que vao participar da
            rotina de uso dos equipamentos do laboratorio.
          </p>
        </aside>

        <form className={styles.form} onSubmit={handleSubmit}>
          {status && <p className={styles[status.type]}>{status.message}</p>}

          <label>
            Nome completo
            <input value={nome} onChange={(event) => setNome(event.target.value)} required />
          </label>

          <label>
            Email institucional
            <input
              type="email"
              value={email}
              onChange={(event) => setEmail(event.target.value)}
              required
            />
          </label>

          <label>
            Senha inicial
            <input
              type="password"
              value={senha}
              onChange={(event) => setSenha(event.target.value)}
              required
              minLength={6}
            />
          </label>

          <div className={styles.twoColumns}>
            <label>
              Telefone
              <input value={telefone} onChange={(event) => setTelefone(event.target.value)} />
            </label>

            <label>
              Perfil
              <select value={papel} onChange={(event) => setPapel(event.target.value)}>
                <option value="aluno">Aluno</option>
                <option value="supervisor">Supervisor</option>
                <option value="admin">Admin</option>
              </select>
            </label>
          </div>

          <button type="submit" disabled={loading}>
            {loading ? 'Cadastrando...' : 'Cadastrar usuario'}
          </button>
        </form>
      </section>
    </main>
  );
}
