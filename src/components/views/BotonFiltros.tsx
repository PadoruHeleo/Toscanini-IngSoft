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
import { FiltrarOrdenesPorFecha } from "./FiltrarOrdenesPorFecha";
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

export function BotonFiltro({ onFiltrar }: Props) {
  const [open, setOpen] = useState(false);
  const [fechaInicio, setFechaInicio] = useState("");
  const [fechaFin, setFechaFin] = useState("");
  const [prioridades, setPrioridades] = useState<string[]>([]);

  const aplicarFiltro = async () => {
    try {
      const ordenes = await invoke<OrdenTrabajo[]>(
        "get_ordenes_trabajo_filtradas",
        { fechaInicio, fechaFin, prioridades }
      );
      onFiltrar(ordenes);
      setOpen(false);
    } catch (err) {
      console.error("Error aplicando filtro:", err);
    }
  };

  const quitarFiltro = async () => {
    try {
      const ordenes = await invoke<OrdenTrabajo[]>("get_ordenes_trabajo");
      onFiltrar(ordenes);
      setFechaInicio("");
      setFechaFin("");
      setPrioridades([]);
      setOpen(false);
    } catch (err) {
      console.error("Error quitando filtro:", err);
    }
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtro
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Filtros de Órdenes</DialogTitle>
          </DialogHeader>

          {/* Filtro por fecha */}
          <FiltrarOrdenesPorFecha
            onChangeFechaInicio={setFechaInicio}
            onChangeFechaFin={setFechaFin}
          />

          {/* Filtro por prioridad */}
          <FiltrarOrdenesPorPrioridad onChange={setPrioridades} />

          <DialogFooter className="mt-4">
            <Button onClick={aplicarFiltro}>Aplicar</Button>
            <Button variant="outline" onClick={quitarFiltro}>
              Quitar filtro
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
