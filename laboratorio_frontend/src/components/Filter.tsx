import styles from './Filter.module.css';
import { FaSearch } from 'react-icons/fa';

type BarraBuscaProps = {
  campos: string[];
  campoSelecionado: string;
  operadorSelecionado: string;
  valor: string;
  onCampoChange: (campo: string) => void;
  onOperadorChange: (operador: string) => void;
  onValorChange: (valor: string) => void;
};

export function BarraBusca({
  campos,
  campoSelecionado,
  operadorSelecionado,
  valor,
  onCampoChange,
  onOperadorChange,
  onValorChange,
}: BarraBuscaProps) {
  return (
    <div className={styles.container}>
      <FaSearch className={styles.icone}></FaSearch>
      <input
        value={valor}
        placeholder="Buscar"
        onChange={(e) => onValorChange(e.target.value)}
      ></input>
      <select
        value={operadorSelecionado}
        onChange={(e) => onOperadorChange(e.target.value)}
      >
        <option value="contem">Contem</option>
        <option value="igual">Igual</option>
      </select>
      <select
        value={campoSelecionado}
        onChange={(e) => onCampoChange(e.target.value)}
      >
        <option value="">Todos os campos</option>
        {campos.map((campo) => (
          <option key={campo} value={campo}>
            {campo.replaceAll('_', ' ')}
          </option>
        ))}
      </select>
    </div>
  );
}
