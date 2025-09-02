import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { FiltrarOrdenesPorFechaClientes } from "./FiltrarOrdenesPorFechaClientes";
import { FiltrarClientesPorCorreo } from "./FiltrarClientesPorCorreo";

interface Cliente {
  cliente_id: number;
  cliente_rut?: string;
  cliente_nombre?: string;
  cliente_correo?: string;
  cliente_telefono?: string;
  cliente_direccion?: string;
  created_by?: number;
  created_at?: string;
}

interface Props {
  onFiltrar: (clientes: Cliente[]) => void;
}

export function UnificarFiltrosClientes({ onFiltrar }: Props) {
  const filtrosIniciales = {
    fecha_inicio: null as string | null,
    fecha_fin: null as string | null,
    correo: null as string[] | null, // 🔹 Cambiado de 'correos' a 'correo'
  };

  const [filtros, setFiltros] = useState(filtrosIniciales);
  const [resetKey, setResetKey] = useState(0);

  const actualizarFiltro = (nuevoFiltro: Partial<typeof filtros>) => {
    setFiltros((prev) => ({ ...prev, ...nuevoFiltro }));
  };

  const aplicarFiltros = async (filtrosActuales = filtros) => {
    try {
      console.log("🔍 Aplicando filtros de clientes:", filtrosActuales);

      const filtrosParaBackend = {
        fecha_inicio: filtrosActuales.fecha_inicio,
        fecha_fin: filtrosActuales.fecha_fin,
        correo: filtrosActuales.correo, // 🔹 Cambiado de 'correos' a 'correo'
      };

      console.log("📤 Enviando al backend:", filtrosParaBackend);

      const clientes = await invoke<Cliente[]>("get_clientes_filtrados", {
        filtros: filtrosParaBackend,
      });

      console.log("📨 Clientes filtrados recibidos:", clientes.length);
      onFiltrar(clientes);
    } catch (err) {
      console.error("❌ Error aplicando filtros:", err);
    }
  };

  useEffect(() => {
    aplicarFiltros(filtros);
  }, [filtros]);

  const hayFiltrosActivos =
    filtros.fecha_inicio !== null ||
    filtros.fecha_fin !== null ||
    filtros.correo !== null; // 🔹 Cambiado de 'correos' a 'correo'

  return (
    <div className="flex gap-2 flex-wrap items-center">
      <FiltrarOrdenesPorFechaClientes
        resetKey={resetKey}
        onChange={({ fechaInicio, fechaFin }) => {
          actualizarFiltro({ fecha_inicio: fechaInicio, fecha_fin: fechaFin });
        }}
      />

      <FiltrarClientesPorCorreo
        resetKey={resetKey}
        onChange={(correos) => actualizarFiltro({ correo: correos })} // 🔹 Cambiado de 'correos' a 'correo'
      />

      {hayFiltrosActivos && (
        <Button
          variant="outline"
          onClick={() => {
            setFiltros(filtrosIniciales);
            setResetKey((prev) => prev + 1);
          }}
        >
          Limpiar
        </Button>
      )}
    </div>
  );
}
