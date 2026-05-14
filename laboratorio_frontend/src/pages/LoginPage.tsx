import { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import api from './../services/api';
import { useAuthStore } from '../store/authStore';
import styles from './LoginPage.module.css';
import logo from '../assets/images/logo.jpg';

export default function LoginPage() {
  const navigate = useNavigate();
  const location = useLocation();
  const usuario = useAuthStore((state) => state.usuario);
  const setUsuario = useAuthStore((state) => state.setUsuario);
  const [email, setEmail] = useState('');
  const [senha, setSenha] = useState('');
  const [erro, setErro] = useState('');

  const from =
    (location.state as { from?: { pathname: string } } | null)?.from
      ?.pathname ?? '/panel';

  useEffect(() => {
    if (usuario) {
      navigate(from, { replace: true });
      return;
    }

    api
      .get('/auth/usuario')
      .then((res) => {
        setUsuario(res.data.data);
        navigate(from, { replace: true });
      })
      .catch(() => undefined);
  }, [from, navigate, setUsuario, usuario]);

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault();
    try {
      await api.post('/auth/login', { email, senha });
      const res = await api.get('/auth/usuario');
      setUsuario(res.data.data);
      navigate(from, { replace: true });
    } catch {
      setErro('Credenciais invalidas. Tente novamente.');
    }
  }

  return (
    <div className={styles.loginPage}>
      <div className={styles.loginCard}>
        <img
          className={styles.logo}
          src={logo}
          alt="Logo Laboratorio de Quimica"
        />

        <h1 className={styles.title}>Acesso ao Sistema</h1>
        <p className={styles.subtitle}>
          Insira suas credenciais para continuar
        </p>

        <form className={styles.loginForm} onSubmit={handleLogin}>
          <div className={styles.inputGroup}>
            <input
              className={styles.loginInput}
              type="email"
              placeholder="E-mail Institucional"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>

          <div className={styles.inputGroup}>
            <input
              className={styles.loginInput}
              type="password"
              placeholder="Senha"
              value={senha}
              onChange={(e) => setSenha(e.target.value)}
              required
            />
          </div>

          {erro && (
            <div className={styles.errorMessage}>
              <span>⚠️</span> {erro}
            </div>
          )}

          <button className={styles.loginBtn} type="submit">
            Entrar
          </button>
        </form>
      </div>
    </div>
  );
}
