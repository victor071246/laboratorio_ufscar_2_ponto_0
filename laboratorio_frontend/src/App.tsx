import { BrowserRouter, Routes, Route } from 'react-router-dom';
import EquipmentRegisterPage from './pages/EquipmentRegisterPage';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import UserRegisterPage from './pages/UserRegisterPage';
import PanelPage from './pages/PanelPage';
import './global.css';

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
      </Routes>
    </BrowserRouter>
  );
}

export default App;
