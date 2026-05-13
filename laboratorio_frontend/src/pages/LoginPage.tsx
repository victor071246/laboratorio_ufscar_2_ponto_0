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
      setErro('Credenciais invalidas');
    }
  }

  return (
    <div className={styles.loginPage}>
      <div className={styles.loginFormDiv}>
        <form className={styles.loginForm} onSubmit={handleLogin}>
          <img className={styles.lgba_logo} src={logo}></img>
          {erro && <p className={styles.floatingLabel}>{erro}</p>}
          <div className={styles.inputs}>
            <input
              className={styles.loginInput}
              type="email"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            ></input>
            <input
              className={styles.loginInput}
              type="password"
              placeholder="Senha"
              value={senha}
              onChange={(e) => setSenha(e.target.value)}
            ></input>
          </div>

          <button className={styles.loginBtn} type="submit">
            Entrar
          </button>
        </form>
      </div>
    </div>
  );
}
