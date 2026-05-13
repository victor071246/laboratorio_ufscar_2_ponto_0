import { useEffect, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { FaArrowLeft } from 'react-icons/fa6';
import { FiLogOut } from 'react-icons/fi';
import api from '../services/api';
import { useAuthStore } from '../store/authStore';
import styles from './Header.module.css';

export default function Header() {
  const usuario = useAuthStore((state) => state.usuario);
  const setUsuario = useAuthStore((state) => state.setUsuario);
  const clearUsuario = useAuthStore((state) => state.clearUsuario);
  const [nome, setNome] = useState<string | null>(usuario?.nome ?? null);
  const navigate = useNavigate();

  useEffect(() => {
    api
      .get('/auth/usuario')
      .then((res) => {
        setUsuario(res.data.data);
        setNome(res.data.data.nome);
      })
      .catch(() => {
        clearUsuario();
        navigate('/login', { replace: true });
      });
  }, [clearUsuario, navigate, setUsuario]);

  async function handleLogout() {
    await api.post('/auth/logout');
    clearUsuario();
    navigate('/login', { replace: true });
  }

  return (
    <header className={styles.topHeader}>
      <Link to="/" className={styles.back}>
        <FaArrowLeft></FaArrowLeft>
      </Link>
      <p className={styles.welcome}>Bem-vindo, {nome ?? '...'}</p>
      <button className={styles.logout} onClick={handleLogout}>
        Sair <FiLogOut fontSize={18}></FiLogOut>
      </button>
    </header>
  );
}
