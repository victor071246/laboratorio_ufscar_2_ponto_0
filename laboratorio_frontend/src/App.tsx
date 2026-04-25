import { BrowserRouter, Routes, Route } from 'react-router-dom';
import EquipmentRegisterPage from './pages/EquipmentRegisterPage';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import UserRegisterPage from './pages/UserRegisterPage';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />}></Route>
        <Route path="/login" element={<LoginPage />}></Route>
        <Route path="/cadastro/usuarios" element={<UserRegisterPage />}></Route>
        <Route path="/cadastro/equipamentos" element={<EquipmentRegisterPage />}></Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
