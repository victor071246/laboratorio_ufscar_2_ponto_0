import { Link } from 'react-router-dom';
import styles from './PanelPage.module.css';
import Footer from '../components/Footer';

export default function PanelPage() {
  return (
    <main className={styles.page}>
      <div className={styles.grid}>
        <Link to="/agendamentos" className={styles.card}>
          Agendamentos
        </Link>
        <Link to="/agendamentos" className={styles.card}>
          Agendamentos
        </Link>
        <Link to="/agendamentos" className={styles.card}>
          Agendamentos
        </Link>
        <Link to="/agendamentos" className={styles.card}>
          Agendamentos
        </Link>
      </div>
      <Footer></Footer>
    </main>
  );
}
