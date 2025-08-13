import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";

import { FiltrarOrdenesPorFecha } from "./FiltrarOrdenesPorFecha";
import { FiltrarOrdenesPorMarca } from "./FiltrarOrdenesPorMarca";
import { FiltrarOrdenesPorModelo } from "./FiltrarOrdenesPorModelo";
import { FiltrarOrdenesPorPrioridad } from "./FiltrarOrdenesPorPrioridad";

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
  const [filtros, setFiltros] = useState({
    fecha_inicio: null as string | null,
    fecha_fin: null as string | null,
    marca: null as string | null,
    modelos: [] as string[], // ← Cambiar a array
    prioridades: [] as string[],
  });

  const actualizarFiltro = (nuevoFiltro: Partial<typeof filtros>) => {
    setFiltros((prev) => ({ ...prev, ...nuevoFiltro }));
  };

  const aplicarFiltros = async (filtrosActuales = filtros) => {
    try {
      console.log("🔍 Aplicando filtros automáticamente:", filtrosActuales);

      // Transformar el formato para el backend
      const filtrosParaBackend = {
        fecha_inicio: filtrosActuales.fecha_inicio,
        fecha_fin: filtrosActuales.fecha_fin,
        marca: filtrosActuales.marca,
        modelos:
          filtrosActuales.modelos.length > 0 ? filtrosActuales.modelos : null, // ← Cambiar a modelos (plural)
        prioridades:
          filtrosActuales.prioridades.length > 0
            ? filtrosActuales.prioridades
            : null,
      };

      console.log("📤 Enviando al backend:", filtrosParaBackend);

      const ordenes = await invoke<OrdenTrabajo[]>(
        "get_ordenes_trabajo_filtradas",
        { filtros: filtrosParaBackend } // ✅ Envolver en objeto con key "filtros"
      );

      console.log("📨 Órdenes filtradas recibidas:", ordenes.length);
      onFiltrar(ordenes);
    } catch (err) {
      console.error("❌ Error aplicando filtros:", err);
    }
  };

  // Aplicar filtros automáticamente cuando cambian
  useEffect(() => {
    console.log("🔄 useEffect disparado - filtros cambiaron:", filtros);
    aplicarFiltros(filtros);
  }, [filtros]);

  const limpiarTodosFiltros = async () => {
    try {
      console.log("🧹 Limpiando todos los filtros");

      // Resetear el estado de filtros
      const filtrosLimpios = {
        fecha_inicio: null,
        fecha_fin: null,
        marca: null,
        modelos: [], // ← Cambiar a array vacío
        prioridades: [],
      };

      setFiltros(filtrosLimpios);

      // Cargar todas las órdenes sin filtros
      const ordenes = await invoke<OrdenTrabajo[]>("get_ordenes_trabajo");
      onFiltrar(ordenes);
    } catch (err) {
      console.error("❌ Error limpiando filtros:", err);
    }
  };

  // Verificar si hay algún filtro activo
  const hayFiltrosActivos =
    filtros.fecha_inicio ||
    filtros.fecha_fin ||
    filtros.marca ||
    filtros.modelos ||
    filtros.prioridades.length > 0;

  return (
    <div className="flex gap-2 flex-wrap items-center">
      <FiltrarOrdenesPorFecha
        onChange={({ fechaInicio, fechaFin }) => {
          console.log("📅 Filtro de fecha cambiado:", {
            fechaInicio,
            fechaFin,
          });
          actualizarFiltro({ fecha_inicio: fechaInicio, fecha_fin: fechaFin });
        }}
      />

      <FiltrarOrdenesPorMarca
        onChange={(marcas) => {
          console.log("🏭 Filtro de marca cambiado:", marcas);
          // Tomar solo la primera marca seleccionada ya que el backend espera un string
          const marca = marcas.length > 0 ? marcas[0] : null;
          actualizarFiltro({ marca });
        }}
      />

      <FiltrarOrdenesPorModelo
        onChange={(modelos) => {
          console.log("📱 Filtro de modelo cambiado:", modelos);
          actualizarFiltro({ modelos }); // ← Enviar array completo
        }}
      />

      <FiltrarOrdenesPorPrioridad
        onChange={(prioridades) => {
          console.log("⚡ Filtro de prioridad cambiado:", prioridades);
          actualizarFiltro({ prioridades });
        }}
      />

      {/* Mostrar botón de limpiar solo si hay filtros activos */}
      {hayFiltrosActivos && (
        <>
          <div className="h-6 w-px bg-gray-300 mx-1" /> {/* Separador visual */}
          <Button
            variant="outline"
            size="sm"
            onClick={limpiarTodosFiltros}
            className="text-gray-600 hover:text-gray-800"
          >
            🧹 Limpiar filtros
          </Button>
        </>
      )}

      {/* Indicador de filtros activos */}
      {hayFiltrosActivos && (
        <span className="text-xs text-blue-600 font-medium">
          Filtros activos
        </span>
      )}
    </div>
  );
}
