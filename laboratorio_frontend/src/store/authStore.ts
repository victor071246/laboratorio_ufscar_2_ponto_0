import { create } from 'zustand';

type UsuarioLogado = {
  nome: string;
  email: string;
  papel: string;
};

type AuthStore = {
  usuario: UsuarioLogado | null;
  setUsuario: (usuario: UsuarioLogado) => void;
  clearUsuario: () => void;
};

export const useAuthStore = create<AuthStore>((set) => ({
  usuario: null,
  setUsuario: (usuario) => set({ usuario }),
  clearUsuario: () => set({ usuario: null }),
}));
