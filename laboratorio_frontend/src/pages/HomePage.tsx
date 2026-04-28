import { Link } from 'react-router-dom';
import styles from './HomePage.module.css';
import logo from '../assets/images/logo.jpg';

const areas = [
  {
    title: 'Equipamentos analiticos',
    text: 'Controle de uso para balancas, pHmetros, espectrofotometros e sistemas de preparo de amostras.',
  },
  {
    title: 'Rotina do laboratorio',
    text: 'Organizacao de disponibilidade, manutencao, historico de uso e registros para apoiar aulas e pesquisa.',
  },
  {
    title: 'Seguranca quimica',
    text: 'Visao rapida de recursos essenciais para manter instrumentos rastreaveis, calibrados e prontos para uso.',
  },
];

export default function HomePage() {
  return (
    <main className={styles.page}>
      <header className={styles.hero}>
        <nav className={styles.nav}>
          <img className={styles.logo} src={logo} alt="Logo LGBA UFSCar" />
          <div className={styles.navActions}>
            <Link className={styles.loginButton} to="/painel/">
              Acesse o sistema
            </Link>
            <Link className={styles.loginButton} to="/cadastro/equipamentos">
              Sobre nós
            </Link>
            <Link className={styles.loginButton} to="/login">
              Login
            </Link>
          </div>
        </nav>

        <section className={styles.heroContent}>
          <p className={styles.eyebrow}>Laboratorio de Quimica - UFSCar</p>
          <h1>
            Gestao de equipamentos para ensino, pesquisa e rotina analitica.
          </h1>
          <p className={styles.heroText}>
            Uma central para acompanhar instrumentos, apoiar agendamentos
            futuros e manter o laboratorio mais organizado para alunos, tecnicos
            e docentes.
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
          <h2>Controle claro para equipamentos que precisam de cuidado.</h2>
        </div>
        <p>
          Em ambientes de quimica, a disponibilidade de equipamentos depende de
          preparo, limpeza, calibracao e manutencao. Esta plataforma organiza
          essas informacoes para reduzir conflitos de uso e preservar a
          qualidade dos ensaios.
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
          <p className={styles.sectionLabel}>Cadastros iniciais</p>
          <h2>Base pronta para quem vai implementar agendamentos depois.</h2>
        </div>
        <div className={styles.workflowActions}>
          <Link to="/cadastro/usuarios">Agendar equipamentos</Link>
          <Link to="/cadastro/equipamentos">Cadastrar equipamentos</Link>
        </div>
      </section>
    </main>
  );
}
