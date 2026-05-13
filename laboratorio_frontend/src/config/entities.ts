export type EntityKey = 'equipamento' | 'agendamento' | 'usuario' | 'ocorrencia';

type EntityConfig = {
  singular: string;
  plural: string;
  label: string;
  endpoint: string;
  panelPath: string;
  consultaPath: string;
  cadastroPath: string;
  hiddenFields?: string[];
};

export const entities: Record<EntityKey, EntityConfig> = {
  equipamento: {
    singular: 'equipamento',
    plural: 'equipamentos',
    label: 'Equipamentos',
    endpoint: '/equipamentos',
    panelPath: '/equipamentos',
    consultaPath: '/equipamentos/consulta',
    cadastroPath: '/cadastro/equipamentos',
  },
  agendamento: {
    singular: 'agendamento',
    plural: 'agendamentos',
    label: 'Agendamentos',
    endpoint: '/agendamentos',
    panelPath: '/agendamentos',
    consultaPath: '/agendamentos/consulta',
    cadastroPath: '/cadastro/agendamentos',
  },
  usuario: {
    singular: 'usuario',
    plural: 'usuarios',
    label: 'Usuarios',
    endpoint: '/usuario',
    panelPath: '/usuarios',
    consultaPath: '/usuarios/consulta',
    cadastroPath: '/cadastro/usuarios',
    hiddenFields: ['senha_hash'],
  },
  ocorrencia: {
    singular: 'ocorrencia',
    plural: 'ocorrencias',
    label: 'Ocorrencias',
    endpoint: '/ocorrencias',
    panelPath: '/ocorrencias',
    consultaPath: '/ocorrencias/consulta',
    cadastroPath: '/cadastro/ocorrencias',
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
