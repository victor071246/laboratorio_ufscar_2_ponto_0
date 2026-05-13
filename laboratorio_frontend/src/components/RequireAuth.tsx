import { useEffect, useState } from 'react';
import { Navigate, Outlet, useLocation } from 'react-router-dom';
import api from '../services/api';
import { useAuthStore } from '../store/authStore';

export default function RequireAuth() {
  const location = useLocation();
  const usuario = useAuthStore((state) => state.usuario);
  const setUsuario = useAuthStore((state) => state.setUsuario);
  const clearUsuario = useAuthStore((state) => state.clearUsuario);
  const [loading, setLoading] = useState(!usuario);

  useEffect(() => {
    if (usuario) return;

    api
      .get('/auth/usuario')
      .then((res) => setUsuario(res.data.data))
      .catch(() => clearUsuario())
      .finally(() => setLoading(false));
  }, [clearUsuario, setUsuario, usuario]);

  if (loading) return null;

  if (!useAuthStore.getState().usuario) {
    return <Navigate to="/login" replace state={{ from: location }} />;
  }

  return <Outlet />;
}
