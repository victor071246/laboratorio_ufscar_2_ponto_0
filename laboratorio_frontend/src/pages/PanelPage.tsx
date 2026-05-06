import { Link } from 'react-router-dom';
import styles from './PanelPage.module.css';
import Footer from '../components/Footer';
import Header from '../components/Header';

export default function PanelPage() {
  return (
    <main className={styles.page}>
      <Header></Header>
      <div className={styles.grid}>
        <Link to="/equipamentos" className={styles.card}>
          Equipamentos
        </Link>
        <Link to="/agendamentos" className={styles.card}>
          Agendamentos
        </Link>
        <Link to="/usuarios" className={styles.card}>
          Usuarios
        </Link>
        <Link to="/ocorrencias" className={styles.card}>
          Ocorrências
        </Link>
      </div>
      <Footer></Footer>
    </main>
  );
}
