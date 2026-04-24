CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TYPE papel_usuario AS ENUM(
    'admin',
    'supervisor',
    'aluno'
);

CREATE TYPE estado_equipamento AS enum(
    'disponivel',
    'em_manutencao',
    'quebrado',
    'reservado',
    'desativado'
);

CREATE TABLE usuario(
    id serial PRIMARY KEY,
    uuid uuid NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    nome varchar(255) NOT NULL,
    email varchar(255) NOT NULL UNIQUE,
    senha_hash text NOT NULL,
    papel papel_usuario NOT NULL DEFAULT 'aluno',
    telefone varchar(20),
    ativo boolean NOT NULL DEFAULT TRUE,
    criado_em timestamptz NOT NULL DEFAULT now(),
    criado_por int REFERENCES usuario(id) ON DELETE SET null    
);

CREATE TABLE equipamento (
    id serial PRIMARY KEY,
    uuid uuid NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    nome varchar(255) NOT NULL,
    descricao text,
    estado estado_equipamento NOT NULL DEFAULT 'disponivel',
    data_aquisicao date,
    peso_kg numeric(8,2),
    largura_cm numeric(8,2),
    altura_cm numeric(8,2),
    profundidade_cm numeric(8,2),
    ultima_vez_disponivel timestamptz,
    ultima_vez_em_manutencao timestamptz,
    criado_em timestamptz NOT NULL DEFAULT now(),
    criado_por int REFERENCES usuario(id) ON DELETE SET null
);

CREATE TYPE tipo_ocorrencia AS enum(
    'manutencao',
    'defeito',
    'acidente',
    'outro'
);

CREATE TABLE ocorrencia (
    id serial PRIMARY KEY,
    uuid uuid NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    equipamento_id int NOT NULL REFERENCES equipamento(id) ON DELETE CASCADE,
    registrado_por int REFERENCES usuario(id) ON DELETE SET NULL,
    tipo tipo_ocorrencia NOT NULL,
    descricao text NOT NULL,
    estado_anterior estado_equipamento,
    removida_por_prazo boolean NOT NULL DEFAULT FALSE,
    resolvida_em timestamptz,
    criado_em timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE bloqueio_horario (
    id serial PRIMARY KEY,
    equipamento_id int REFERENCES equipamento(id) ON DELETE CASCADE,
    data_inicio timestamptz,
    data_fim timestamptz,
    dia_semana SMALLINT CHECK (dia_semana BETWEEN 0 AND 6),
    hora_inicio time,
    hora_fim time,
    motivo text,
    criado_em timestamptz NOT NULL DEFAULT now(),
    criado_por int REFERENCES usuario(id) ON DELETE SET NULL
);

CREATE TYPE status_agendamento AS ENUM (
    'pendente',
    'confirmado',
    'em_uso',
    'concluido',
    'cancelado'
);

CREATE TABLE agendamento(
    id serial PRIMARY KEY,
    uuid uuid NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    equipamento_id int NOT NULL REFERENCES equipamento(id) ON DELETE CASCADE,
    status status_agendamento NOT NULL DEFAULT 'pendente',
    notificar_email boolean NOT NULL DEFAULT true,
    notificar_whatsapp boolean NOT NULL DEFAULT FALSE,
    observacao text,
    criado_em timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE slot (
    id serial PRIMARY KEY,
    equipamento_id int NOT NULL REFERENCES equipamento(id) ON DELETE CASCADE,
    agendamento_id int REFERENCES agendamento(id) ON DELETE SET NULL,
    data_hora timestamptz NOT NULL,
    criado_em timestamptz NOT NULL DEFAULT now(),
    UNIQUE (equipamento_id, DATA_hora)
);

CREATE TABLE avise_me (
    id serial PRIMARY KEY,
    usuario_id int NOT NULL REFERENCES usuario(id) ON DELETE CASCADE,
    equipamento_id int NOT NULL REFERENCES equipamento(id) ON DELETE CASCADE,
    ativo boolean NOT NULL DEFAULT TRUE,
    criado_em timestamptz NOT NULL DEFAULT now(),
    UNIQUE (usuario_id, equipamento_id)
);

CREATE TABLE log_equipamento (
    id serial PRIMARY KEY,
    equipamento_id int NOT NULL REFERENCES equipamento(id) ON DELETE CASCADE,
    alterado_por int REFERENCES usuario(id) ON DELETE SET NULL,
    dados_anteriores jsonb NOT NULL,
    dados_novos jsonb NOT NULL,
    alterado_em timestamptz NOT NULL DEFAULT now()
);

ALTER TABLE log_equipamento
RENAME COLUMN dados_novos TO dados_posteriores;

ALTER TABLE log_equipamento
RENAME COLUMN dados_posteriores TO dados_inseridos;

CREATE TABLE log_usuario(
    id serial PRIMARY key,
    usuario_id int NOT NULL REFERENCES usuario(id) ON DELETE CASCADE,
    alterado_por int REFERENCES usuario(id) ON DELETE SET NULL,
    dados_anteriores jsonb NOT NULL,
    dados_novos jsonb NOT NULL,
    alterado_em timestamptz NOT NULL DEFAULT now()
);

ALTER TABLE log_usuario
RENAME COLUMN dados_novos TO dados_inseridos;

CREATE TABLE token_reset_senha (
    id serial PRIMARY KEY,
    usuario_id int NOT NULL REFERENCES usuario(id) ON DELETE CASCADE,
    token_hash text NOT NULL UNIQUE,
    expira_em timestamptz NOT NULL,
    usado boolean NOT NULL DEFAULT FALSE,
    criado_em timestamptz NOT NULL DEFAULT now()
);

---
--- ADIÇÃO EXCLUSIVA: ÍNDICES PARA PERFORMANCE
---

CREATE INDEX idx_agendamento_status ON agendamento(status);
CREATE INDEX idx_slot_data_hora ON slot(data_hora);
CREATE INDEX idx_equipamento_estado ON equipamento(estado);
CREATE INDEX idx_log_equip_data ON log_equipamento(alterado_em);
CREATE INDEX idx_log_user_data ON log_usuario(alterado_em);

ALTER TABLE agendamento
ADD COLUMN usuario_id int NOT NULL REFERENCES usuario(id) ON DELETE CASCADE,
ADD COLUMN data_inicio timestamptz NOT NULL,
ADD COLUMN data_fim timestamptz NOT NULL;

DROP TABLE slot;

CREATE INDEX idx_agendamento_usuario ON agendamento(usuario_id);
CREATE INDEX idx_agendamento_horario ON agendamento(equipamento_id, data_inicio, data_fim);

ALTER TABLE agendamento
ALTER COLUMN usuario_id SET NOT NULL,
ALTER COLUMN data_inicio SET NOT NULL,
ALTER COLUMN data_fim SET NOT NULL;
