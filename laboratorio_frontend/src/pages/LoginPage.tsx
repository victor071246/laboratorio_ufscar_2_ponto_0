import { useState } from 'react';
import api from './../services/api';
import styles from './LoginPage.module.css';
import logo from '../assets/images/logo.jpg';

export default function LoginPage() {
  const [email, setEmail] = useState('');
  const [senha, setSenha] = useState('');
  const [erro, setErro] = useState('');

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault();
    try {
      await api.post('/auth/login', { email, senha });
      window.location.href = '/';
    } catch {
      setErro('Credenciais inválidas. Tente novamente.');
    }
  }

  return (
    <div className={styles.loginPage}>
      <div className={styles.loginCard}>
        <img className={styles.logo} src={logo} alt="Logo Laboratório de Química" />
        
        <h1 className={styles.title}>Acesso ao Sistema</h1>
        <p className={styles.subtitle}>Insira suas credenciais para continuar</p>

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
