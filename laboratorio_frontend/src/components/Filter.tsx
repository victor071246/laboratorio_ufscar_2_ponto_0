import { useEffect, useState } from 'react';
import api from '../services/api';
import styles from './Filter.module.css';
import { FaSearch } from 'react-icons/fa';

export function BarraBusca({ tabela }: { tabela: string }) {
  const [campoSelecionado, setCampoSelecionado] = useState('');
  const [operadorSelecionado, setOperadorSelecionado] = useState('=');
  const [valor, setValor] = useState('');
  const [campos, setCampos] = useState<string[]>([]);

  const operadores = ['>', '>=', '<=', '<', '==', '='];

  useEffect(() => {
    api.get(`/${tabela}/campos`).then((res) => {
      const chaves = Object.keys(res.data.dados[0]);
      setCampos(chaves);
    });
  }, [tabela]);

  useEffect(() => {
    if (!campoSelecionado || !valor.trim()) return;

    api.get(`/${tabela}`, {
      params: { campo: campoSelecionado, operador: operadorSelecionado, valor },
    });
  });

  return (
    <div className={styles.container}>
      <FaSearch className={styles.icone}></FaSearch>
      <input value={valor} onChange={(e) => setValor(e.target.value)}></input>
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
    </div>
  );
}
