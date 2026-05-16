import { useState } from 'react';
import { Link } from 'react-router-dom';
import { FiMenu, FiX } from 'react-icons/fi';
import styles from './HomePage.module.css';
import logo from '../assets/images/logo.jpg';

const areas = [
  {
    title: 'Patrimonio instrumental',
    text: 'Cadastro e rastreamento de balancas, espectrofotometros, centrifugas e demais equipamentos com estado, dimensoes e historico de manutencao.',
  },
  {
    title: 'Agendamentos',
    text: 'Reserva de equipamentos com controle de conflito de horario, notificacoes e registro de uso por usuario.',
  },
  {
    title: 'Ocorrencias',
    text: 'Registro de defeitos e manutencoes com rastreamento de estado anterior e data de resolucao.',
  },
];

export default function HomePage() {
  const [menuOpen, setMenuOpen] = useState(false);
  return (
    <main className={styles.page}>
      <header className={styles.hero}>
        <nav className={styles.nav}>
          <img className={styles.logo} src={logo} alt="Logo LGBA UFSCar" />
          <button
            className={styles.hamburger}
            onClick={() => setMenuOpen(!menuOpen)}
            type="button"
          >
            {menuOpen ? <FiX size={32} /> : <FiMenu size={32} />}
          </button>
          <div
            className={`${styles.navActions} ${menuOpen ? styles.navActionsOpen : ''}`}
          >
            <Link className={styles.navTextLink} to="/login">
              Login
            </Link>
            <Link className={styles.navPrimaryButton} to="/panel/">
              Acesse o sistema
            </Link>
          </div>
        </nav>
        <section className={styles.heroContent}>
          <p className={styles.eyebrow}>
            Laboratorio de Quimica Analitica — UFSCar
          </p>
          <h1>
            Sistema de gestao de equipamentos, agendamentos e ocorrencias para
            suporte a rotina de ensino e pesquisa.
          </h1>
          <p className={styles.heroText}>
            Controle centralizado de instrumentos analiticos, historico de uso e
            disponibilidade em tempo real para alunos, tecnicos e docentes.
          </p>
          <div className={styles.heroActions}>
            <Link className={styles.primaryButton} to="/login">
              Entrar no sistema
            </Link>
            <Link className={styles.outlineButton} to="/cadastro/equipamentos">
              Novo equipamento
            </Link>
          </div>
        </section>
      </header>
      <section className={styles.overview}>
        <div>
          <p className={styles.sectionLabel}>Operacao do laboratorio</p>
          <h2>
            Instrumentos analiticos exigem controle rigoroso de disponibilidade,
            calibracao e manutencao.
          </h2>
        </div>
        <p>
          Esta plataforma centraliza o cadastro de equipamentos, agendamentos e
          ocorrencias, reduzindo conflitos de uso e mantendo o historico
          necessario para rastreabilidade e qualidade dos ensaios.
        </p>
      </section>
      <section className={styles.cards}>
        {areas.map((area) => (
          <article className={styles.card} key={area.title}>
            <h3>{area.title}</h3>
            <p>{area.text}</p>
          </article>
        ))}
      </section>
      <section className={styles.workflow}>
        <div>
          <p className={styles.sectionLabel}>Acesso rapido</p>
          <h2>
            Cadastros e agendamentos disponiveis para usuarios autenticados.
          </h2>
        </div>
        <div className={styles.workflowActions}>
          <Link to="/cadastro/agendamentos">Agendar equipamentos</Link>
          <Link to="/cadastro/equipamentos">Cadastrar equipamentos</Link>
        </div>
      </section>
    </main>
  );
}
