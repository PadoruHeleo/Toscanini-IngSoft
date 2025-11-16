import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";

interface Props {
  resetKey?: number;
  onChange: (estados: string[] | null) => void;
}

export function FiltrarEquiposPorEstado({ resetKey, onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [estadosDisponibles, setEstadosDisponibles] = useState<string[]>([]);
  const [seleccionados, setSeleccionados] = useState<string[]>([]);
  const [busqueda, setBusqueda] = useState("");

  // Cargar estados al abrir
  useEffect(() => {
    const fetchEstados = async () => {
      try {
        const lista = await invoke<string[]>("get_estados_ordenes_trabajo");
        setEstadosDisponibles(lista);
      } catch (err) {
        console.error("❌ Error cargando estados:", err);
      }
    };
    fetchEstados();
  }, [resetKey]);

  // Reset automático cuando cambia resetKey
  useEffect(() => {
    setSeleccionados([]);
    setBusqueda("");
  }, [resetKey]);

  const toggleSeleccion = (estado: string) => {
    setSeleccionados((prev) =>
      prev.includes(estado)
        ? prev.filter((e) => e !== estado)
        : [...prev, estado]
    );
  };

  const limpiar = () => {
    setSeleccionados([]);
    setBusqueda("");
    onChange(null);
  };

  const aplicarFiltro = () => {
    onChange(seleccionados.length > 0 ? seleccionados : null);
    setOpen(false);
  };

  const estadosFiltrados = estadosDisponibles.filter((e) =>
    e.toLowerCase().includes(busqueda.toLowerCase())
  );

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por estado
        {seleccionados.length > 0 && (
          <span className="ml-1 bg-blue-100 text-blue-800 px-1 rounded text-xs flex items-center gap-1">
            {seleccionados.length}
            <button
              onClick={(e) => {
                e.stopPropagation();
                limpiar();
              }}
              className="ml-1 text-red-500 hover:text-red-700 font-bold text-lg leading-none hover:bg-red-200 rounded-full px-1"
            >
              ×
            </button>
          </span>
        )}
      </Button>

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Seleccione que estados quiere ver</DialogTitle>
          </DialogHeader>

          <Input
            placeholder="Buscar estado..."
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            className="mb-2"
          />

          <div className="max-h-64 overflow-y-auto space-y-2">
            {estadosFiltrados.map((estado) => (
              <div key={estado} className="flex items-center space-x-2">
                <Checkbox
                  checked={seleccionados.includes(estado)}
                  onCheckedChange={() => toggleSeleccion(estado)}
                />
                <span>{estado}</span>
              </div>
            ))}
          </div>

          <DialogFooter className="flex gap-2">
            <Button onClick={aplicarFiltro}>
              Aplicar {seleccionados.length > 0 && `(${seleccionados.length})`}
            </Button>
            <Button variant="outline" onClick={limpiar}>
              Limpiar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
