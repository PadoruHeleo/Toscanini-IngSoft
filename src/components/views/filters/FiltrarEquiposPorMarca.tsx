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
  onChange: (marcas: string[] | null) => void;
}

export function FiltrarEquiposPorMarca({ resetKey, onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [marcasDisponibles, setMarcasDisponibles] = useState<string[]>([]);
  const [seleccionados, setSeleccionados] = useState<string[]>([]);
  const [busqueda, setBusqueda] = useState("");

  // Cargar marcas al abrir
  useEffect(() => {
    const fetchMarcas = async () => {
      try {
        const lista = await invoke<string[]>("get_equipos_marcas");
        setMarcasDisponibles(lista);
      } catch (err) {
        console.error("❌ Error cargando marcas:", err);
      }
    };
    fetchMarcas();
  }, [resetKey]);

  // Reset automático cuando cambia resetKey
  useEffect(() => {
    setSeleccionados([]);
    setBusqueda("");
  }, [resetKey]);

  const toggleSeleccion = (marca: string) => {
    setSeleccionados((prev) =>
      prev.includes(marca) ? prev.filter((m) => m !== marca) : [...prev, marca]
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

  const marcasFiltradas = marcasDisponibles.filter((m) =>
    m.toLowerCase().includes(busqueda.toLowerCase())
  );

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por marca
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
            <DialogTitle>Seleccione que marcas quiere ver</DialogTitle>
          </DialogHeader>

          <Input
            placeholder="Buscar marca..."
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            className="mb-2"
          />

          <div className="max-h-64 overflow-y-auto space-y-2">
            {marcasFiltradas.map((marca) => (
              <div key={marca} className="flex items-center space-x-2">
                <Checkbox
                  checked={seleccionados.includes(marca)}
                  onCheckedChange={() => toggleSeleccion(marca)}
                />
                <span>{marca}</span>
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
