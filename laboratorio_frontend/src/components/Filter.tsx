import { useState } from 'react';

type Filtro = {
    tabela: string;
};

export function BarraBusca({ filtros, onBuscar }) {
    const [campoSelecionado, setCampoSelecionado] = useState('');
    const [operadorSelecionado, setOperadorSelecionado] = useState('=');
    const [valor, setValor] = useState('');

    const operadores = ['>', '>=', '<=', '<', '==', '='];

    const handleBuscar = () => {
        if (!campoSelecionado || !valor) return;
        onBuscar(campoSelecionado, operadorSelecionado, valor);
    };

    return <div></div>;
}
