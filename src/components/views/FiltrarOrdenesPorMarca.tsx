import { useEffect, useState } from "react";
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

export function FiltrarOrdenesPorMarca({ onFiltrar }: Props) {
  const [marcas, setMarcas] = useState<string[]>([]);
  const [marcaSeleccionada, setMarcaSeleccionada] = useState<string>("");
  const [open, setOpen] = useState(false);

  // Cargar marcas cuando se abre el modal
  useEffect(() => {
    if (!open) return;
    (async () => {
      try {
        const data = await invoke<string[]>("get_marcas_equipos");
        setMarcas(data);
      } catch (err) {
        console.error("Error cargando marcas:", err);
        setMarcas([]);
      }
    })();
  }, [open]);

  const aplicarFiltro = async () => {
    if (!marcaSeleccionada) return;
    try {
      const ordenes = await invoke<OrdenTrabajo[]>(
        "get_ordenes_trabajo_por_marca",
        { marca: marcaSeleccionada }
      );
      onFiltrar(ordenes);
      setOpen(false);
    } catch (err) {
      console.error("Error filtrando por marca:", err);
    }
  };

  const quitarFiltro = async () => {
    try {
      const ordenes = await invoke<OrdenTrabajo[]>("get_ordenes_trabajo");
      onFiltrar(ordenes);
      setMarcaSeleccionada("");
      setOpen(false);
    } catch (err) {
      console.error("Error quitando filtro:", err);
    }
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Marca
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="sm:max-w-md">
          <DialogHeader>
            <DialogTitle>Filtrar por Marca</DialogTitle>
          </DialogHeader>

          <div className="space-y-3 mt-2">
            {marcas.length === 0 ? (
              <p className="text-sm text-gray-500">
                No hay marcas registradas.
              </p>
            ) : (
              <div className="space-y-2">
                {marcas.map((m) => (
                  <label key={m} className="flex items-center gap-2">
                    <input
                      type="radio"
                      name="marca"
                      value={m}
                      checked={marcaSeleccionada === m}
                      onChange={(e) => setMarcaSeleccionada(e.target.value)}
                    />
                    <span>{m}</span>
                  </label>
                ))}
              </div>
            )}
          </div>

          <DialogFooter className="mt-4">
            <Button onClick={aplicarFiltro} disabled={!marcaSeleccionada}>
              Aplicar
            </Button>
            <Button variant="outline" onClick={quitarFiltro}>
              Quitar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
