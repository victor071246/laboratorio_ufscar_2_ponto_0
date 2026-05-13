import { Link } from 'react-router-dom';
import Header from '../components/Header';
import Footer from '../components/Footer';
import { entities } from '../config/entities';
import type { EntityKey } from '../config/entities';
import styles from '../pages/PanelPage.module.css';

export default function EntityPanelPage({ tabela }: { tabela: EntityKey }) {
  const entity = entities[tabela];

  return (
    <main className={styles.page}>
      <Header></Header>
      <div className={styles.grid}>
        <Link to={entity.consultaPath} className={styles.card}>
          Consultar {entity.singular}
        </Link>
        <Link to={entity.cadastroPath} className={styles.card}>
          Cadastrar {entity.singular}
        </Link>
      </div>
      <Footer></Footer>
    </main>
  );
}
