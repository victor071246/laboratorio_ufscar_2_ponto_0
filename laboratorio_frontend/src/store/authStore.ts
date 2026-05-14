import { create } from 'zustand';
import api from '../services/api';

type UsuarioLogado = {
  nome: string;
  email: string;
  papel: string;
};

type AuthStore = {
  usuario: UsuarioLogado | null;
  setUsuario: (usuario: UsuarioLogado) => void;
  clearUsuario: () => void;
  logout: () => Promise<void>;
};

export const useAuthStore = create<AuthStore>((set) => ({
  usuario: null,
  setUsuario: (usuario) => set({ usuario }),
  clearUsuario: () => set({ usuario: null }),
  logout: async () => {
    await api.post('/auth/logout').catch(() => undefined);
    set({ usuario: null });
  },
}));
