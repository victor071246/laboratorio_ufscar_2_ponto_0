export type EntityKey =
  | 'equipamentos'
  | 'agendamentos'
  | 'usuario'
  | 'ocorrencias';

type EntityConfig = {
  singular: string;
  plural: string;
  label: string;
  endpoint: string;
  panelPath: string;
  consultaPath: string;
  cadastroPath: string;
  detalhePath: string;
  hiddenFields?: string[];
  editableFields: string[];
};

export const entities: Record<EntityKey, EntityConfig> = {
  equipamentos: {
    singular: 'equipamento',
    plural: 'equipamentos',
    label: 'Equipamentos',
    endpoint: '/equipamentos',
    panelPath: '/equipamentos',
    consultaPath: '/equipamentos/consulta',
    cadastroPath: '/cadastro/equipamentos',
    detalhePath: '/equipamento/:id',
    hiddenFields: ['id', 'uuid'],
    editableFields: [
      'nome',
      'descricao',
      'estado',
      'data_aquisicao',
      'peso_kg',
      'largura_cm',
      'altura_cm',
      'comprimento_cm',
    ],
  },
  agendamentos: {
    singular: 'agendamento',
    plural: 'agendamentos',
    label: 'Agendamentos',
    endpoint: '/agendamentos',
    panelPath: '/agendamentos',
    consultaPath: '/agendamentos/consulta',
    cadastroPath: '/cadastro/agendamentos',
    detalhePath: '/agendamentos/:id',
    hiddenFields: ['id', 'uuid'],
    editableFields: [
      'status',
      'data_inicio',
      'data_fim',
      'notificar_email',
      'notificar_whatsapp',
      'observacao',
    ],
  },
  usuario: {
    singular: 'usuario',
    plural: 'usuarios',
    label: 'Usuarios',
    endpoint: '/usuario',
    panelPath: '/usuarios',
    consultaPath: '/usuarios/consulta',
    cadastroPath: '/cadastro/usuarios',
    detalhePath: '/usuario/:id',
    hiddenFields: ['id', 'uuid', 'senha_hash'],
    editableFields: ['nome', 'email', 'telefone', 'papel', 'ativo'],
  },
  ocorrencias: {
    singular: 'ocorrencia',
    plural: 'ocorrencias',
    label: 'Ocorrencias',
    endpoint: '/ocorrencias',
    panelPath: '/ocorrencias',
    consultaPath: '/ocorrencias/consulta',
    cadastroPath: '/cadastro/ocorrencias',
    detalhePath: '/ocorrencias/:id',
    hiddenFields: ['id', 'uuid'],
    editableFields: ['descricao'],
  },
};

export function formatColumnName(value: string) {
  return value.replaceAll('_', ' ');
}

export function formatCellValue(value: unknown) {
  if (value === null || value === undefined || value === '') return '-';
  if (typeof value === 'boolean') return value ? 'Sim' : 'Nao';
  if (typeof value === 'string' && /^\d{4}-\d{2}-\d{2}T/.test(value)) {
    return new Date(value).toLocaleString('pt-BR');
  }
  return String(value);
}
