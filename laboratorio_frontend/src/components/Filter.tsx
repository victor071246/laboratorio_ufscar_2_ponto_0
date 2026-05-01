import { useState } from 'react';

type CampoFiltro = {
  label: string;
  placeholder: string;
  campo: string;
  tipo: 'texto' | 'numero';
};
