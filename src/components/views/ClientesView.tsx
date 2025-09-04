import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { ViewTitle } from "@/components/ViewTitle";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Search, Edit, Trash2 } from "lucide-react";
import { ClienteFormDialog } from "./ClienteFormDialog";
import { useToastContext } from "@/contexts/ToastContext";
import { useAuth } from "@/contexts/AuthContext";
import { UnificarFiltrosClientes } from "./UnificarFiltrosClientes";

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

export function ClientesView() {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [clientes, setClientes] = useState<Cliente[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingCliente, setEditingCliente] = useState<Cliente | null>(null);
  const searchInputRef = useRef<HTMLInputElement>(null);
  const searchTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Estado para forzar actualización de filtros
  const [refreshFilters, setRefreshFilters] = useState(0);

  // 🔥 Función para validar que solo contenga texto (sin números)
  const isValidText = (text: string): boolean => {
    // Solo permite: letras (con acentos), espacios, apostrofes, guiones
    const textOnlyRegex = /^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s'\-]*$/;
    return textOnlyRegex.test(text);
  };

  // 🔥 Manejar cambios en el input de búsqueda (solo texto)
  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;

    // Solo actualizar si es texto válido (sin números)
    if (isValidText(value)) {
      setSearchTerm(value);
    }
  };

  // 🔥 Prevenir entrada de números al escribir
  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    // Prevenir números (0-9)
    if (/[0-9]/.test(e.key)) {
      e.preventDefault();
    }
  };

  // ✅ Carga inicial de clientes (sin filtros)
  useEffect(() => {
    const loadInitialClientes = async () => {
      try {
        setLoading(true);
        const clientesData = await invoke<Cliente[]>("get_clientes");
        setClientes(clientesData);
      } catch (error) {
        console.error("Error cargando clientes:", error);
        showError(
          "Error al cargar clientes",
          "No se pudieron cargar los clientes."
        );
      } finally {
        setLoading(false);
      }
    };

    loadInitialClientes();
  }, []);

  // ✅ Manejo del término de búsqueda con debounce (sin cargar clientes directamente)
  useEffect(() => {
    if (searchTimeoutRef.current) {
      clearTimeout(searchTimeoutRef.current);
    }

    // El debounce solo actualiza el estado, no carga clientes
    // UnificarFiltrosClientes se encarga de aplicar el filtro
    searchTimeoutRef.current = setTimeout(() => {
      // El searchTerm se pasa como prop a UnificarFiltrosClientes
    }, 150);

    return () => {
      if (searchTimeoutRef.current) {
        clearTimeout(searchTimeoutRef.current);
      }
    };
  }, [searchTerm]);

  const handleClienteAdded = () => {
    setShowAddForm(false);
    setRefreshFilters((prev) => prev + 1);
  };

  const handleClienteUpdated = () => {
    setEditingCliente(null);
    setRefreshFilters((prev) => prev + 1);
  };

  const handleEditCliente = (cliente: Cliente) => {
    setEditingCliente(cliente);
  };

  const handleDeleteCliente = async (cliente: Cliente) => {
    if (!user) return;

    const confirmDelete = window.confirm(
      `¿Está seguro que desea eliminar al cliente "${cliente.cliente_nombre}"?\n\nEsta acción no se puede deshacer.`
    );

    if (!confirmDelete) return;

    try {
      const result = await invoke<boolean>("delete_cliente", {
        clienteId: cliente.cliente_id,
        deletedBy: user.usuario_id,
      });

      if (result) {
        success(
          "Cliente eliminado",
          `${cliente.cliente_nombre} ha sido eliminado exitosamente.`
        );
        setRefreshFilters((prev) => prev + 1);
      } else {
        showError("Error", "No se pudo eliminar el cliente.");
      }
    } catch (error) {
      console.error("Error eliminando cliente:", error);
      showError(
        "Error al eliminar cliente",
        typeof error === "string" ? error : "Ha ocurrido un error inesperado."
      );
    }
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return "N/A";
    return new Date(dateString).toLocaleDateString("es-CL");
  };

  // ✅ Función que recibe los clientes filtrados desde UnificarFiltrosClientes
  const handleClientesFiltrados = (clientesFiltrados: Cliente[]) => {
    setClientes(clientesFiltrados);
    setLoading(false);
  };

  if (loading) {
    return (
      <div className="p-4">
        <ViewTitle />
        <div className="text-center py-8">Cargando clientes...</div>
      </div>
    );
  }

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <ViewTitle />
        <Button onClick={() => setShowAddForm(true)}>Agregar Cliente</Button>
      </div>

      {/* Barra de búsqueda */}
      <div className="flex items-center space-x-2 mb-4">
        <div className="relative flex-1 max-w-sm">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            ref={searchInputRef}
            placeholder="Buscar por nombre..."
            value={searchTerm}
            onChange={handleSearchChange}
            onKeyPress={handleKeyPress}
            className="pl-8"
            title="Solo se permiten letras y espacios"
          />
        </div>

        {/* ✅ Filtros unificados - pasamos searchTerm y función para recibir resultados */}
        <div className="flex-shrink-0">
          <UnificarFiltrosClientes
            key={refreshFilters}
            searchTerm={searchTerm}
            onFiltrar={handleClientesFiltrados}
          />
        </div>
      </div>

      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>RUT</TableHead>
              <TableHead>Nombre</TableHead>
              <TableHead>Correo</TableHead>
              <TableHead>Teléfono</TableHead>
              <TableHead>Dirección</TableHead>
              <TableHead>Fecha Registro</TableHead>
              <TableHead className="text-right">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {clientes.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={7}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm
                    ? "No se encontraron clientes"
                    : "No hay clientes registrados"}
                </TableCell>
              </TableRow>
            ) : (
              clientes.map((cliente) => (
                <TableRow key={cliente.cliente_id}>
                  <TableCell className="font-medium">
                    {cliente.cliente_rut || "N/A"}
                  </TableCell>
                  <TableCell>{cliente.cliente_nombre || "N/A"}</TableCell>
                  <TableCell>{cliente.cliente_correo || "N/A"}</TableCell>
                  <TableCell>{cliente.cliente_telefono || "N/A"}</TableCell>
                  <TableCell className="max-w-xs truncate">
                    {cliente.cliente_direccion || "N/A"}
                  </TableCell>
                  <TableCell>{formatDate(cliente.created_at)}</TableCell>
                  <TableCell className="text-right">
                    <div className="flex gap-1 justify-end">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleEditCliente(cliente)}
                        className="text-gray-600 hover:text-gray-700"
                        title="Editar cliente"
                      >
                        <Edit className="h-3 w-3" />
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleDeleteCliente(cliente)}
                        className="text-red-600 hover:text-red-700"
                        title="Eliminar cliente"
                      >
                        <Trash2 className="h-3 w-3" />
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      {/* Total de clientes */}
      <div className="mt-4 text-sm text-gray-600">
        Total: {clientes.length} cliente{clientes.length !== 1 ? "s" : ""}
        {searchTerm && (
          <span className="ml-2 text-blue-600">
            (filtrado por: "{searchTerm}")
          </span>
        )}
      </div>

      {/* Dialog para agregar cliente */}
      <ClienteFormDialog
        open={showAddForm}
        onOpenChange={setShowAddForm}
        onClienteAdded={handleClienteAdded}
      />

      {/* Dialog para editar cliente */}
      {editingCliente && (
        <ClienteFormDialog
          open={!!editingCliente}
          onOpenChange={(open: boolean) => !open && setEditingCliente(null)}
          onClienteAdded={handleClienteUpdated}
          cliente={editingCliente}
          isEditing={true}
        />
      )}
    </div>
  );
}
