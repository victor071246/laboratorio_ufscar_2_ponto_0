import { useEffect, useState } from 'react';

type Filtro = {
    tabela: string;
};

export function BarraBusca({ tabela }: { tabela: string }) {
    const [campoSelecionado, setCampoSelecionado] = useState('');
    const [operadorSelecionado, setOperadorSelecionado] = useState('=');
    const [valor, setValor] = useState('');

    const operadores = ['>', '>=', '<=', '<', '==', '='];

    useEffect(() => {}, [tabela]);

    return <div></div>;
}
