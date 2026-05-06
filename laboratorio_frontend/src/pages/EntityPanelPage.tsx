import { Link } from 'react-router-dom';
import Header from '../components/Header';
import Footer from '../components/Footer';
import styles from '../pages/PanelPage.module.css';

export default function EntityPanelPage({ tabela }: { tabela: string }) {
  return (
    <main className={styles.page}>
      <Header></Header>
      <div className={styles.grid}>
        <Link to={`/${tabela}s/consulta`} className={styles.card}>
          Consultar {tabela}
        </Link>
        <Link to={`/cadastro/${tabela}s/`} className={styles.card}>
          {' '}
          Cadastrar {tabela}
        </Link>
      </div>
      <Footer></Footer>
    </main>
  );
}
