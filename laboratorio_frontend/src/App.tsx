import { BrowserRouter, Routes, Route } from 'react-router-dom';
import EquipmentRegisterPage from './pages/EquipmentRegisterPage';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import UserRegisterPage from './pages/UserRegisterPage';
import ScheduleRegisterPage from './pages/ScheduleRegisterPage';
import OccurrenceRegisterPage from './pages/OccurrenceRegisterPage';
import PanelPage from './pages/PanelPage';
import './global.css';
import EntityPanelPage from './pages/EntityPanelPage';
import DataPage from './pages/DataPage';
import RequireAuth from './components/RequireAuth';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/login" element={<LoginPage />} />
        <Route element={<RequireAuth />}>
          <Route path="/cadastro/usuarios" element={<UserRegisterPage />} />
          <Route
            path="/cadastro/equipamentos"
            element={<EquipmentRegisterPage />}
          />
          <Route
            path="/cadastro/agendamentos"
            element={<ScheduleRegisterPage />}
          />
          <Route
            path="/cadastro/ocorrencias"
            element={<OccurrenceRegisterPage />}
          />
          <Route path="/panel" element={<PanelPage />} />
          <Route
            path="/equipamentos"
            element={<EntityPanelPage tabela="equipamentos" />}
          />
          <Route
            path="/agendamentos"
            element={<EntityPanelPage tabela="agendamentos" />}
          />
          <Route
            path="/usuarios"
            element={<EntityPanelPage tabela="usuario" />}
          />
          <Route
            path="/ocorrencias"
            element={<EntityPanelPage tabela="ocorrencias" />}
          />
          <Route
            path="/equipamentos/consulta"
            element={<DataPage tabela="equipamentos" />}
          />
          <Route
            path="/agendamentos/consulta"
            element={<DataPage tabela="agendamentos" modo="agendamento" />}
          />
          <Route
            path="/usuarios/consulta"
            element={<DataPage tabela="usuario" />}
          />
          <Route
            path="/ocorrencias/consulta"
            element={<DataPage tabela="ocorrencias" />}
          />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
