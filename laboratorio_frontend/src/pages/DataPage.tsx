import { useEffect, useState } from 'react';
import { BarraBusca } from '../components/Filter';
import { useNavigate } from 'react-router-dom';
import Header from '../components/Header';
import Footer from '../components/Footer';
import api from '../services/api';
import {
  entities,
  formatCellValue,
  formatColumnName,
} from '../config/entities';
import type { EntityKey } from '../config/entities';
import styles from './DataPage.module.css';

type Row = Record<string, unknown>;

export default function DataPage({
  tabela,
  modo,
}: {
  tabela: EntityKey;
  modo?: 'agendamento' | 'default';
}) {
  const entity = entities[tabela];
  const [dados, setDados] = useState<Row[]>([]);
  const [resultados, setResultados] = useState<Row[]>([]);
  const [campos, setCampos] = useState<string[]>([]);
  const [campoSelecionado, setCampoSelecionado] = useState('');
  const [operadorSelecionado, setOperadorSelecionado] = useState('contem');
  const [valor, setValor] = useState('');
  const [loading, setLoading] = useState(true);
  const [erro, setErro] = useState('');
  const navigate = useNavigate();
  const [mostrarCancelados, setMostrarCancelados] = useState(false);

  function handleClick(item: Row) {
    const id = item.uuid ?? item.id;
    if (modo === 'agendamento') {
      navigate(`/agendamentos/${id}/grid`);
      return;
    }
    navigate(entity.detalhePath.replace(':id', String(id)));
  }

  useEffect(() => {
    setLoading(true);
    setErro('');
    setValor('');
    setCampoSelecionado('');
    setOperadorSelecionado('contem');

    Promise.all([
      api.get(entity.endpoint),
      api.get(`${entity.endpoint}/campos`),
    ])
      .then(([dadosRes, camposRes]) => {
        const lista = dadosRes.data.data ?? [];
        const listaCancelados =
          tabela === 'agendamentos' && !mostrarCancelados
            ? lista.filter((item: Row) => item.status !== 'cancelado')
            : lista;
        setDados(listaCancelados);
        setResultados(listaCancelados);
        const hiddenFields = entity.hiddenFields ?? [];
        setCampos(
          (camposRes.data.data ?? []).filter(
            (campo: string) => !hiddenFields.includes(campo),
          ),
        );
      })
      .catch(() => {
        setDados([]);
        setResultados([]);
        setCampos([]);
        setErro(`Nao foi possivel carregar ${entity.label.toLowerCase()}.`);
      })
      .finally(() => setLoading(false));
  }, [entity.endpoint, entity.label, mostrarCancelados]);

  useEffect(() => {
    const termo = valor.trim().toLowerCase();
    if (!termo) {
      setResultados(dados);
      return;
    }

    if (campoSelecionado) {
      setLoading(true);
      setErro('');
      api
        .get(`${entity.endpoint}/buscar`, {
          params: {
            campo: campoSelecionado,
            operador: operadorSelecionado,
            valor,
          },
        })
        .then((res) => setResultados(res.data.data ?? []))
        .catch(() => {
          setResultados([]);
          setErro(`Nao foi possivel filtrar ${entity.label.toLowerCase()}.`);
        })
        .finally(() => setLoading(false));
      return;
    }

    const filtrados = dados.filter((item) => {
      const chaves = Object.keys(item);
      return chaves.some((chave) =>
        formatCellValue(item[chave]).toLowerCase().includes(termo),
      );
    });
    setResultados(filtrados);
  }, [
    campoSelecionado,
    dados,
    entity.endpoint,
    entity.label,
    operadorSelecionado,
    valor,
  ]);

  return (
    <main className={styles.page}>
      <Header />
      <section className={styles.content}>
        <div className={styles.filterArea}>
          <BarraBusca
            campos={campos}
            campoSelecionado={campoSelecionado}
            operadorSelecionado={operadorSelecionado}
            valor={valor}
            onCampoChange={setCampoSelecionado}
            onOperadorChange={setOperadorSelecionado}
            onValorChange={setValor}
          />
          {tabela === 'agendamentos' && (
            <label>
              <input
                type="checkbox"
                checked={mostrarCancelados}
                onChange={(e) => setMostrarCancelados(e.target.checked)}
              ></input>
              Mostrar cancelados
            </label>
          )}
        </div>

        {loading ? (
          <div className={styles.emptyState}>Carregando...</div>
        ) : erro ? (
          <div className={styles.emptyState}>{erro}</div>
        ) : resultados.length > 0 ? (
          <div className={styles.tableWrapper}>
            <table className={styles.table}>
              <thead>
                <tr>
                  {campos.map((chave) => (
                    <th key={chave}>{formatColumnName(chave)}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {resultados.map((item, index) => (
                  <tr
                    key={String(item.uuid ?? item.id ?? index)}
                    onClick={() => handleClick(item)}
                    style={{ cursor: 'pointer' }}
                  >
                    {campos.map((campo) => (
                      <td key={campo}>{formatCellValue(item[campo])}</td>
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
