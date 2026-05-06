import { BrowserRouter, Routes, Route } from 'react-router-dom';
import EquipmentRegisterPage from './pages/EquipmentRegisterPage';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import UserRegisterPage from './pages/UserRegisterPage';
import PanelPage from './pages/PanelPage';
import './global.css';
import EntityPanelPage from './pages/EntityPanelPage';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />}></Route>
        <Route path="/login" element={<LoginPage />}></Route>
        <Route path="/cadastro/usuarios" element={<UserRegisterPage />}></Route>
        <Route
          path="/cadastro/equipamentos"
          element={<EquipmentRegisterPage />}
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
      </Routes>
    </BrowserRouter>
  );
}

export default App;
