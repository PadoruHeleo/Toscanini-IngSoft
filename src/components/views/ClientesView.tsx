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
import { ViewTitle } from "@/components/layout/ViewTitle";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Search, Edit, Trash2, History } from "lucide-react";
import { ClienteFormDialog } from "./dialogs/ClienteFormDialog";
import { ClienteHistorialDialog } from "./dialogs/ClienteHistorialDialog";
import { useToastContext } from "@/contexts/ToastContext";
import { useAuth } from "@/contexts/AuthContext";
import { UnificarFiltrosClientes } from "./filters/UnificarFiltrosClientes";
import { useClientePermissions } from "@/hooks/use-permissions";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

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
  const { canCreateCliente, canEditCliente, userRole } =
    useClientePermissions();
  const [clientes, setClientes] = useState<Cliente[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingCliente, setEditingCliente] = useState<Cliente | null>(null);
  const [historialCliente, setHistorialCliente] = useState<Cliente | null>(
    null
  );
  const [refreshFilters, setRefreshFilters] = useState(0);

  // --- Estados para eliminar cliente ---
  const [clienteToDelete, setClienteToDelete] = useState<Cliente | null>(null);
  const [deleteMotivo, setDeleteMotivo] = useState("");
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);

  const searchInputRef = useRef<HTMLInputElement>(null);
  const searchTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Validación de texto (sin números)
  const isValidText = (text: string) =>
    /^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s'\-]*$/.test(text);

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    if (isValidText(value)) setSearchTerm(value);
  };

  const handleClearSearch = () => setSearchTerm("");

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (/[0-9]/.test(e.key)) e.preventDefault();
  };

  // Cargar clientes inicial
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

  // Debounce búsqueda
  useEffect(() => {
    if (searchTimeoutRef.current) clearTimeout(searchTimeoutRef.current);

    searchTimeoutRef.current = setTimeout(() => {
      // UnificarFiltrosClientes se encarga de filtrar
    }, 150);

    return () => {
      if (searchTimeoutRef.current) clearTimeout(searchTimeoutRef.current);
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

  const handleEditCliente = (cliente: Cliente) => setEditingCliente(cliente);
  const handleVerHistorial = (cliente: Cliente) => setHistorialCliente(cliente);

  // --- Abrir modal de eliminación ---
  const handleOpenDeleteDialog = (cliente: Cliente) => {
    setClienteToDelete(cliente);
    setDeleteMotivo("");
    setShowDeleteDialog(true);
  };

  // --- Confirmar eliminación ---
  const handleConfirmDelete = async () => {
    if (!clienteToDelete || !user) return;

    try {
      const result = await invoke<boolean>("delete_cliente", {
        request: {
          cliente_id: clienteToDelete.cliente_id,
          deleted_by: user.usuario_id,
          motivo: deleteMotivo,
          deleted_at: new Date().toISOString(),
        },
      });

      if (result) {
        success(
          "Cliente eliminado",
          `${clienteToDelete.cliente_nombre} ha sido eliminado exitosamente.`
        );
        setShowDeleteDialog(false);
        setClienteToDelete(null);
        setDeleteMotivo("");
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

  const formatDate = (dateString?: string) =>
    dateString ? new Date(dateString).toLocaleDateString("es-CL") : "N/A";

  const handleClientesFiltrados = (clientesFiltrados: Cliente[]) => {
    setClientes(clientesFiltrados);
    setLoading(false);
  };

  if (loading) {
    return (
      <div className="px-6 pt-6">
        <ViewTitle />
        <div className="text-center py-8">Cargando clientes...</div>
      </div>
    );
  }

  return (
    <div className="px-6 pt-6 space-y-6">
      <div className="flex justify-between items-center">
        <ViewTitle />
        {canCreateCliente && (
          <Button onClick={() => setShowAddForm(true)}>Agregar Cliente</Button>
        )}
        {!canCreateCliente && (
          <div className="text-sm text-gray-500">
            Rol actual: {userRole} - Solo visualización permitida
          </div>
        )}
      </div>

      {/* Barra de búsqueda */}
      <div className="flex items-center space-x-2">
        <div className="relative w-auto">
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
        <div className="flex-grow min-w-0">
          <UnificarFiltrosClientes
            key={refreshFilters}
            searchTerm={searchTerm}
            onFiltrar={handleClientesFiltrados}
            onClearSearch={handleClearSearch}
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
                      {canEditCliente && (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleEditCliente(cliente)}
                          className="text-gray-600 hover:text-gray-700"
                          title="Editar cliente"
                        >
                          <Edit className="h-3 w-3" />
                        </Button>
                      )}
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleVerHistorial(cliente)}
                        className="text-blue-600 hover:text-blue-700"
                        title="Ver historial del cliente"
                      >
                        <History className="h-3 w-3" />
                      </Button>
                      {canEditCliente && (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleOpenDeleteDialog(cliente)}
                          className="text-red-600 hover:text-red-700"
                          title="Eliminar cliente"
                        >
                          <Trash2 className="h-3 w-3" />
                        </Button>
                      )}
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

      {/* Dialog para ver historial del cliente */}
      <ClienteHistorialDialog
        open={historialCliente !== null}
        onOpenChange={(open) => !open && setHistorialCliente(null)}
        cliente={historialCliente}
      />

      {/* Dialog de eliminación */}
      <Dialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Confirmar Eliminación</DialogTitle>
            <DialogDescription>
              ¿Está seguro que desea eliminar al cliente "
              {clienteToDelete?.cliente_nombre}"?
            </DialogDescription>
          </DialogHeader>

          <div className="mt-4">
            <label className="block text-sm font-medium mb-1">
              Motivo de eliminación
            </label>
            <textarea
              value={deleteMotivo}
              onChange={(e) => setDeleteMotivo(e.target.value)}
              placeholder="Ingrese motivo..."
              className="w-full border rounded-md p-2 mt-1"
              rows={4}
            />
          </div>

          <DialogFooter className="gap-2 mt-4">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowDeleteDialog(false)}
            >
              Cancelar
            </Button>
            <Button onClick={handleConfirmDelete} disabled={!deleteMotivo}>
              Confirmar Eliminación
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
