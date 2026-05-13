import { BarraBusca } from '../components/Filter';
import { useFiltroStore } from '../store/filtroStore';
import { useNavigate } from 'react-router-dom';
import Header from '../components/Header';
import Footer from '../components/Footer';
import styles from './DataPage.module.css';

export default function DataPage({
  tabela,
  modo,
}: {
  tabela: string;
  modo?: 'agendamento' | 'default';
}) {
  const resultados = useFiltroStore((s) => s.resultados);
  const navigate = useNavigate();

  function handleClick(id: number) {
    if (modo === 'agendamento') {
      navigate(`/agendamentos/${id}/grid`);
    }
    navigate(`/equipamentos/${id}`);
  }

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
                  <tr key={index} onClick={() => handleClick(item.id)}>
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
