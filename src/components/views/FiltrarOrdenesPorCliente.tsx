import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { invoke } from "@tauri-apps/api/core";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";

interface Props {
  onChange: (clientes: string[]) => void;
}

export function FiltrarOrdenesPorCliente({ onChange }: Props) {
  const [open, setOpen] = useState(false);
  const [seleccionadas, setSeleccionadas] = useState<string[]>([]);
  const [clientesDisponibles, setClientesDisponibles] = useState<string[]>([]);
  const [clientesFiltrados, setClientesFiltrados] = useState<string[]>([]);
  const [textoBusqueda, setTextoBusqueda] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Cargar clientes cuando se abre el diálogo
  useEffect(() => {
    if (open) {
      cargarClientes();
    }
  }, [open]);

  // Filtrar clientes según el texto de búsqueda
  useEffect(() => {
    if (textoBusqueda.trim() === "") {
      setClientesFiltrados(clientesDisponibles);
    } else {
      const filtrados = clientesDisponibles.filter((cliente) =>
        cliente.toLowerCase().includes(textoBusqueda.toLowerCase())
      );
      setClientesFiltrados(filtrados);
    }
  }, [textoBusqueda, clientesDisponibles]);

  const cargarClientes = async () => {
    setLoading(true);
    setError(null);

    try {
      console.log("🔄 Cargando clientes disponibles...");
      const clientes = await invoke<string[]>("get_clientes_disponibles");
      console.log("👥 Clientes cargados:", clientes);
      setClientesDisponibles(clientes);
      setClientesFiltrados(clientes);
    } catch (err) {
      console.error("❌ Error cargando clientes:", err);
      setError("Error al cargar los clientes");
      // Fallback a datos de ejemplo
      const clientesEjemplo = [
        "Laboratorio ABC",
        "Clínica XYZ",
        "Hospital Central",
      ];
      setClientesDisponibles(clientesEjemplo);
      setClientesFiltrados(clientesEjemplo);
    } finally {
      setLoading(false);
    }
  };

  const toggleCliente = (cliente: string) => {
    setSeleccionadas((prev) =>
      prev.includes(cliente)
        ? prev.filter((x) => x !== cliente)
        : [...prev, cliente]
    );
  };

  const aplicar = () => {
    console.log("👥 Aplicando filtro de clientes:", seleccionadas);
    onChange(seleccionadas);
    setOpen(false);
  };

  const limpiar = () => {
    console.log("🧹 Limpiando filtro de clientes");
    setSeleccionadas([]);
    onChange([]);
    setOpen(false);
  };

  const handleOpenChange = (newOpen: boolean) => {
    setOpen(newOpen);
    if (!newOpen) {
      // Limpiar búsqueda al cerrar
      setTextoBusqueda("");
    }
  };

  return (
    <>
      <Button variant="outline" onClick={() => setOpen(true)}>
        Filtrar por Cliente
        {seleccionadas.length > 0 && (
          <span className="ml-1 bg-green-100 text-green-800 px-1 rounded text-xs">
            {seleccionadas.length}
          </span>
        )}
      </Button>

      <Dialog open={open} onOpenChange={handleOpenChange}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Selecciona Clientes</DialogTitle>
          </DialogHeader>

          <div className="space-y-3 mt-2">
            {/* Campo de búsqueda */}
            <div>
              <Input
                placeholder="Buscar cliente..."
                value={textoBusqueda}
                onChange={(e) => setTextoBusqueda(e.target.value)}
                disabled={loading}
                className="w-full"
              />
            </div>

            {/* Lista de clientes */}
            <div className="max-h-60 overflow-y-auto border rounded p-2">
              {loading ? (
                <div className="flex items-center justify-center py-4">
                  <span className="text-gray-500">Cargando clientes...</span>
                </div>
              ) : error ? (
                <div className="text-red-500 text-sm py-2">⚠️ {error}</div>
              ) : clientesFiltrados.length === 0 ? (
                <div className="text-gray-500 text-sm py-2">
                  {textoBusqueda.trim()
                    ? "No se encontraron clientes que coincidan"
                    : "No se encontraron clientes"}
                </div>
              ) : (
                <div className="space-y-1">
                  {clientesFiltrados.map((cliente) => (
                    <label
                      key={cliente}
                      className="flex items-center gap-2 hover:bg-gray-50 p-2 rounded cursor-pointer transition-colors"
                    >
                      <input
                        type="checkbox"
                        checked={seleccionadas.includes(cliente)}
                        onChange={() => toggleCliente(cliente)}
                        className="rounded"
                      />
                      <span className="flex-1 text-sm">{cliente}</span>
                    </label>
                  ))}
                </div>
              )}
            </div>

            {/* Mostrar seleccionados */}
            {seleccionadas.length > 0 && (
              <div className="text-sm text-gray-600">
                <strong>Seleccionados:</strong> {seleccionadas.join(", ")}
              </div>
            )}
          </div>

          <DialogFooter className="gap-2">
            <Button onClick={aplicar} disabled={loading}>
              Aplicar {seleccionadas.length > 0 && `(${seleccionadas.length})`}
            </Button>
            <Button variant="outline" onClick={limpiar} disabled={loading}>
              Limpiar
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
}
