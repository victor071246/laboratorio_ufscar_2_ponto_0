import { useEffect, useState } from 'react';
import api from '../services/api';
import styles from './Filter.module.css';
import { FaSearch } from 'react-icons/fa';
import { useFiltroStore } from '../store/filtroStore';

export function BarraBusca({ tabela }: { tabela: string }) {
  const [campoSelecionado, setCampoSelecionado] = useState('');
  const [operadorSelecionado, setOperadorSelecionado] = useState('=');
  const [valor, setValor] = useState('');
  const [campos, setCampos] = useState<string[]>([]);
  const setResultados = useFiltroStore((s) => s.setResultados);

  const operadores = ['>', '>=', '<=', '<', '==', '='];

  useEffect(() => {
    api
      .get(`/${tabela}/campos`)
      .then((res) => {
        if (Array.isArray(res.data.data)) {
          setCampos(res.data.data);
        }
      })
      .catch((err) => {
        console.error('Erro ao buscar campos: ', err);
      });
  }, [tabela]);

  useEffect(() => {
    api
      .get(`/${tabela}`)
      .then((res) => {
        if (Array.isArray(res.data.data)) setResultados(res.data.data);
      })
      .catch((err) => console.error('Erro ao listar: ', err));
  }, [tabela]);

  useEffect(() => {
    if (!campoSelecionado || !valor.trim()) return;

    api
      .get(`/${tabela}`, {
        params: {
          campo: campoSelecionado,
          operador: operadorSelecionado,
          valor,
        },
      })
      .then((res) => {
        if (Array.isArray(res.data.data)) setResultados(res.data.data);
      });
  }, [campoSelecionado, operadorSelecionado, valor, tabela]);

  return (
    <div className={styles.container}>
      <FaSearch className={styles.icone}></FaSearch>
      <select
        value={campoSelecionado}
        onChange={(e) => setCampoSelecionado(e.target.value)}
      >
        {campos.map((c) => (
          <option key={c} value={c}>
            {c}
          </option>
        ))}
      </select>

      <select
        value={operadorSelecionado}
        onChange={(e) => setOperadorSelecionado(e.target.value)}
      >
        {operadores.map((op) => (
          <option key={op} value={op}>
            {op}
          </option>
        ))}
      </select>
      <input value={valor} onChange={(e) => setValor(e.target.value)}></input>
    </div>
  );
}
