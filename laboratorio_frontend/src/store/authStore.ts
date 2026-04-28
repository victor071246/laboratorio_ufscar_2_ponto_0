type AuthStore = {
  usuario: { nome: string; email: string; papel: string } | null;
  carregarUsuario: () => Promise<void>;
  logout: () => void;
};
