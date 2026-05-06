import { BarraBusca } from '../components/Filter';
import { useFiltroStore } from '../store/filtroStore';
import Header from '../components/Header';
import Footer from '../components/Footer';
import styles from './DataPage.module.css';

export default function DataPage({ tabela }: { tabela: string }) {
  const resultados = useFiltroStore((s) => s.resultados);

  return (
    <main className={styles.page}>
      <Header />
      <section className={styles.content}>
        <div className={styles.filterArea}>
          <BarraBusca tabela={tabela} />
        </div>

        {resultados.length > 0 ? (
          <div className={styles.tableWrapper}>
            <table className={styles.table}>
              <thead>
                <tr>
                  {Object.keys(resultados[0]).map((chave) => (
                    <th key={chave}>{chave}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {resultados.map((item, index) => (
                  <tr key={index}>
                    {Object.values(item).map((valor, i) => (
                      <td key={i}>{String(valor)}</td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <div className={styles.emptyState}>Nenhum resultado encontrado.</div>
        )}
      </section>
      <Footer />
    </main>
  );
}
