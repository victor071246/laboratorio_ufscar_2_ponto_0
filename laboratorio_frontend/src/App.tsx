import { BrowserRouter, Routes, Route } from 'react-router-dom';
import EquipmentRegisterPage from './pages/EquipmentRegisterPage';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import UserRegisterPage from './pages/UserRegisterPage';
import PanelPage from './pages/PanelPage';
import './global.css';
import DataPage from './pages/DataPage';

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
          element={<DataPage tabela="equipamento"></DataPage>}
        ></Route>
        <Route
          path="/agendamentos"
          element={<DataPage tabela="agendamento"></DataPage>}
        ></Route>
        <Route
          path="/usuarios"
          element={<DataPage tabela="usuario"></DataPage>}
        ></Route>
        <Route
          path="/ocorrencias"
          element={<DataPage tabela="ocorrencia"></DataPage>}
        ></Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
