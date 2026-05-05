import { create } from 'zustand';

//any nesse caso está correto porque é qualquer objeto de qualquer lugar
type FiltroStore = {
  resultados: Record<string, any>[];
  setResultados: (dados: Record<string, any>[]) => void;
};

export const useFiltroStore = create<FiltroStore>((set) => ({
  resultados: [],
  setResultados: (dados) => set({ resultados: dados }),
}));
