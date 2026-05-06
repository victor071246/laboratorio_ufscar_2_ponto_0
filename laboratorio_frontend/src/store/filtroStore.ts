import { create } from 'zustand';

//any nesse caso está correto porque é qualquer objeto de qualquer lugar
type FiltroStore = {
  tabela: string;
  resultados: Record<string, any>[];
  setTabela: (tabela: string) => void;
  setResultados: (dados: Record<string, any>[]) => void;
};

export const useFiltroStore = create<FiltroStore>((set) => ({
  tabela: '',
  resultados: [],
  setTabela: (tabela) => set({ tabela }),
  setResultados: (dados) => set({ resultados: dados }),
}));
