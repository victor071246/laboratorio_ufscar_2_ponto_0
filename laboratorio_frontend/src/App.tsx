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
        <Route path="/" element={<HomePage />}></Route>
        <Route path="/login" element={<LoginPage />}></Route>
        <Route element={<RequireAuth />}>
          <Route
            path="/cadastro/usuarios"
            element={<UserRegisterPage />}
          ></Route>
          <Route
            path="/cadastro/equipamentos"
            element={<EquipmentRegisterPage />}
          ></Route>
          <Route
            path="/cadastro/agendamentos"
            element={<ScheduleRegisterPage />}
          ></Route>
          <Route
            path="/cadastro/ocorrencias"
            element={<OccurrenceRegisterPage />}
          ></Route>
          <Route path="/panel" element={<PanelPage></PanelPage>}></Route>
          <Route
            path="/equipamentos"
            element={<EntityPanelPage tabela="equipamento"></EntityPanelPage>}
          ></Route>
          <Route
            path="/agendamentos"
            element={<EntityPanelPage tabela="agendamento"></EntityPanelPage>}
          ></Route>
          <Route
            path="/usuarios"
            element={<EntityPanelPage tabela="usuario"></EntityPanelPage>}
          ></Route>
          <Route
            path="/ocorrencias"
            element={<EntityPanelPage tabela="ocorrencia"></EntityPanelPage>}
          ></Route>
          <Route
            path="/equipamentos/consulta"
            element={<DataPage tabela="equipamento"></DataPage>}
          ></Route>
          <Route
            path="/agendamentos/consulta"
            element={<DataPage tabela="agendamento"></DataPage>}
          ></Route>
          <Route
            path="/usuarios/consulta"
            element={<DataPage tabela="usuario"></DataPage>}
          ></Route>
          <Route
            path="/ocorrencias/consulta"
            element={<DataPage tabela="ocorrencia"></DataPage>}
          ></Route>
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
