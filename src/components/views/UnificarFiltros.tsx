import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";

import { FiltrarOrdenesPorFecha } from "./FiltrarOrdenesPorFecha";
import { FiltrarOrdenesPorMarca } from "./FiltrarOrdenesPorMarca";
import { FiltrarOrdenesPorModelo } from "./FiltrarOrdenesPorModelo";
import { FiltrarOrdenesPorPrioridad } from "./FiltrarOrdenesPorPrioridad";
import { FiltrarOrdenesPorCliente } from "./FiltrarOrdenesPorCliente";

interface OrdenTrabajo {
  orden_id: number;
  orden_codigo?: string;
  orden_desc?: string;
  prioridad?: string;
  estado?: string;
  has_garantia?: boolean;
  created_at?: string;
}

interface Props {
  onFiltrar: (ordenes: OrdenTrabajo[]) => void;
}

export function UnificarFiltros({ onFiltrar }: Props) {
  const filtrosIniciales = {
    fecha_inicio: null as string | null,
    fecha_fin: null as string | null,
    marcas: [] as string[],
    modelos: [] as string[],
    prioridades: [] as string[],
    clientes: [] as string[],
  };

  const [filtros, setFiltros] = useState(filtrosIniciales);
  const [resetKey, setResetKey] = useState(0); // 🔹 usado para resetear filtros hijos

  const actualizarFiltro = (nuevoFiltro: Partial<typeof filtros>) => {
    setFiltros((prev) => ({ ...prev, ...nuevoFiltro }));
  };

  const aplicarFiltros = async (filtrosActuales = filtros) => {
    try {
      console.log("🔍 Aplicando filtros automáticamente:", filtrosActuales);

      const filtrosParaBackend = {
        fecha_inicio: filtrosActuales.fecha_inicio,
        fecha_fin: filtrosActuales.fecha_fin,
        marcas:
          filtrosActuales.marcas.length > 0 ? filtrosActuales.marcas : null,
        modelos:
          filtrosActuales.modelos.length > 0 ? filtrosActuales.modelos : null,
        prioridades:
          filtrosActuales.prioridades.length > 0
            ? filtrosActuales.prioridades
            : null,
        clientes:
          filtrosActuales.clientes.length > 0 ? filtrosActuales.clientes : null,
      };

      console.log("📤 Enviando al backend:", filtrosParaBackend);

      const ordenes = await invoke<OrdenTrabajo[]>(
        "get_ordenes_trabajo_filtradas",
        { filtros: filtrosParaBackend }
      );

      console.log("📨 Órdenes filtradas recibidas:", ordenes.length);
      onFiltrar(ordenes);
    } catch (err) {
      console.error("❌ Error aplicando filtros:", err);
    }
  };

  useEffect(() => {
    console.log("🔄 useEffect disparado - filtros cambiaron:", filtros);
    aplicarFiltros(filtros);
  }, [filtros]);

  const hayFiltrosActivos =
    filtros.fecha_inicio !== null ||
    filtros.fecha_fin !== null ||
    filtros.marcas.length > 0 ||
    filtros.modelos.length > 0 ||
    filtros.prioridades.length > 0 ||
    filtros.clientes.length > 0;

  return (
    <div className="flex gap-2 flex-wrap items-center">
      <FiltrarOrdenesPorFecha
        resetKey={resetKey}
        onChange={({ fechaInicio, fechaFin }) => {
          actualizarFiltro({ fecha_inicio: fechaInicio, fecha_fin: fechaFin });
        }}
      />

      <FiltrarOrdenesPorCliente
        resetKey={resetKey}
        onChange={(clientes) => {
          actualizarFiltro({ clientes });
        }}
      />

      <FiltrarOrdenesPorMarca
        resetKey={resetKey}
        onChange={(marcas) => {
          actualizarFiltro({ marcas });
        }}
      />

      <FiltrarOrdenesPorModelo
        resetKey={resetKey}
        onChange={(modelos) => {
          actualizarFiltro({ modelos });
        }}
      />

      <FiltrarOrdenesPorPrioridad
        resetKey={resetKey}
        onChange={(prioridades) => {
          actualizarFiltro({ prioridades });
        }}
      />

      {hayFiltrosActivos && (
        <Button
          variant="outline"
          onClick={() => {
            setFiltros(filtrosIniciales);
            setResetKey((prev) => prev + 1); // 🔹 dispara el reseteo de hijos
          }}
        >
          Limpiar
        </Button>
      )}
    </div>
  );
}
