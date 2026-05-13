import { useEffect, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { FaArrowLeft } from 'react-icons/fa6';
import { FiLogOut } from 'react-icons/fi';
import api from '../services/api';
import styles from './Header.module.css';

export default function Header() {
  const [nome, setNome] = useState<string | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    api
      .get('/auth/usuario')
      .then((res) => setNome(res.data.data.nome))
      .catch(() => {});
    //.catch(() => navigate('/login'));
  }, []);

  async function handleLogout() {
    await api.post('/auth/logout');
    navigate('/login');
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
