import { useState } from 'react';
import api from './../services/api';
import styles from './LoginPage.module.css';

export default function LoginPage() {
  const [email, setEmail] = useState('');
  const [senha, setSenha] = useState('');
  const [erro, setErro] = useState('');

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault();
    try {
      await api.post('/login', { email, senha });
      window.location.href = '/';
    } catch {
      setErro('Credenciais inválidas');
    }
  }

  return (
    <div className={styles.loginPage}>
      <div className={styles.loginFormDiv}>
        <form className={styles.loginForm} onSubmit={handleLogin}>
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
