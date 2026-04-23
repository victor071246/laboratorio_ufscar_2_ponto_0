import { BrowserRouter, Routes, Route } from 'react-router-dom';
import LoginPage from './pages/LoginPage';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<div>home</div>}></Route>
        <Route path="/login" element={<LoginPage></LoginPage>}></Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
