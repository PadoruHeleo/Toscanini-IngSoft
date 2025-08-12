import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { invoke } from "@tauri-apps/api/core";

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

export function FiltrarOrdenesPorFecha({ onFiltrar }: Props) {
  const [fechaInicio, setFechaInicio] = useState("");
  const [fechaFin, setFechaFin] = useState("");
  const [isOpen, setIsOpen] = useState(false);

  const aplicarFiltro = async () => {
    if (!fechaInicio || !fechaFin) return;
    try {
      const ordenesFiltradas = await invoke<OrdenTrabajo[]>(
        "get_ordenes_trabajo_por_fecha",
        { fechaInicio, fechaFin }
      );
      onFiltrar(ordenesFiltradas);
      setIsOpen(false);
    } catch (error) {
      console.error("Error filtrando órdenes por fecha:", error);
    }
  };

  const limpiarFiltro = async () => {
    try {
      const todas = await invoke<OrdenTrabajo[]>("get_ordenes_trabajo");
      onFiltrar(todas);
      setFechaInicio("");
      setFechaFin("");
      setIsOpen(false);
    } catch (error) {
      console.error("Error cargando todas las órdenes:", error);
    }
  };

  return (
    <>
      {/* Botón que abre el modal */}
      <Button variant="outline" onClick={() => setIsOpen(true)}>
        Filtro
      </Button>

      {/* Modal */}
      <Dialog open={isOpen} onOpenChange={setIsOpen}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Filtrar por Fecha</DialogTitle>
          </DialogHeader>

          <div className="space-y-3">
            <div>
              <label className="block text-sm font-medium">Desde</label>
              <input
                type="date"
                value={fechaInicio}
                onChange={(e) => setFechaInicio(e.target.value)}
                className="border rounded p-1 w-full"
              />
            </div>

            <div>
              <label className="block text-sm font-medium">Hasta</label>
              <input
                type="date"
                value={fechaFin}
                onChange={(e) => setFechaFin(e.target.value)}
                className="border rounded p-1 w-full"
              />
            </div>
          </div>

          <DialogFooter className="mt-4">
            <Button onClick={aplicarFiltro}>Aplicar</Button>
            <Button variant="outline" onClick={limpiarFiltro}>
              Quitar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
