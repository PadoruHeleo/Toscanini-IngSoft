import { useState, useEffect } from "react";
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
import { Search, Edit, Trash2 } from "lucide-react";
import { UsuarioFormDialog } from "./dialogs/UsuarioFormDialog";
import { useToastContext } from "@/contexts/ToastContext";
import { useAuth } from "@/contexts/AuthContext";

interface Usuario {
  usuario_id: number;
  usuario_rut?: string;
  usuario_nombre?: string;
  usuario_correo?: string;
  usuario_telefono?: string;
  usuario_rol?: string;
  created_by?: number;
  created_at?: string;
  is_active?: boolean;
}

export function UsuarioView() {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [usuarios, setUsuarios] = useState<Usuario[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingUsuario, setEditingUsuario] = useState<Usuario | null>(null);

  const loadUsuarios = async () => {
    try {
      setLoading(true);
      let usuariosData: Usuario[];

      if (searchTerm.trim()) {
        usuariosData = await invoke<Usuario[]>("search_usuarios", {
          searchTerm: searchTerm.trim(),
        });
      } else {
        usuariosData = await invoke<Usuario[]>("get_usuarios");
      }

      setUsuarios(usuariosData);
    } catch (error) {
      console.error("Error cargando usuarios:", error);
      showError(
        "Error al cargar usuarios",
        "No se pudieron cargar los usuarios."
      );
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadUsuarios();
  }, [searchTerm]);

  const handleUsuarioAdded = () => {
    loadUsuarios();
    setShowAddForm(false);
  };

  const handleUsuarioUpdated = () => {
    loadUsuarios();
    setEditingUsuario(null);
  };

  const handleEditUsuario = (usuario: Usuario) => {
    setEditingUsuario(usuario);
  };

  const handleDeleteUsuario = async (usuario: Usuario) => {
    if (!user) return;

    const confirmDelete = window.confirm(
      `¿Está seguro que desea eliminar al usuario "${usuario.usuario_nombre}"?\n\nEsta acción no se puede deshacer.`
    );

    if (!confirmDelete) return;

    try {
      const result = await invoke<boolean>("delete_usuario", {
        usuarioId: usuario.usuario_id,
        deletedBy: user.usuario_id,
      });

      if (result) {
        success(
          "Usuario eliminado",
          `${usuario.usuario_nombre} ha sido eliminado exitosamente.`
        );
        loadUsuarios();
      } else {
        showError("Error", "No se pudo eliminar el usuario.");
      }
    } catch (error) {
      console.error("Error eliminando usuario:", error);
      showError(
        "Error al eliminar usuario",
        typeof error === "string" ? error : "Ha ocurrido un error inesperado."
      );
    }
  };

  function getRolLabel(rol?: string) {
    switch (rol) {
      case "admin":
        return "Administrador";
      case "recepcion":
        return "Recepcionista";
      case "tecnico":
        return "Técnico";
      default:
        return "N/A";
    }
  }

  if (loading) {
    return (
      <div className="p-4">
        <ViewTitle />
        <div className="text-center py-8">Cargando usuarios...</div>
      </div>
    );
  }

  return (
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <ViewTitle />
        <Button onClick={() => setShowAddForm(true)}>Agregar Usuario</Button>
      </div>
      {/* Barra de búsqueda */}
      <div className="flex items-center space-x-2 mb-4">
        <div className="relative flex-1 max-w-sm">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Buscar usuarios..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="pl-8"
          />
        </div>
        {searchTerm && (
          <Button variant="outline" onClick={() => setSearchTerm("")}>
            Limpiar
          </Button>
        )}
      </div>
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>RUT</TableHead>
              <TableHead>Nombre</TableHead>
              <TableHead>Correo</TableHead>
              <TableHead>Teléfono</TableHead>
              <TableHead>Rol</TableHead>
              <TableHead>Habilitado</TableHead>
              <TableHead className="text-right">Acciones</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {usuarios.length === 0 ? (
              <TableRow>
                <TableCell
                  colSpan={6}
                  className="text-center py-8 text-gray-500"
                >
                  {searchTerm
                    ? "No se encontraron usuarios"
                    : "No hay usuarios registrados"}
                </TableCell>
              </TableRow>
            ) : (
              usuarios.map((usuario) => (
                <TableRow key={usuario.usuario_id}>
                  <TableCell className="font-medium">
                    {usuario.usuario_rut || "N/A"}
                  </TableCell>
                  <TableCell>{usuario.usuario_nombre || "N/A"}</TableCell>
                  <TableCell>{usuario.usuario_correo || "N/A"}</TableCell>
                  <TableCell>{usuario.usuario_telefono || "N/A"}</TableCell>
                  <TableCell>{getRolLabel(usuario.usuario_rol)}</TableCell>
                  <TableCell>{usuario.is_active ? "Sí" : "No"}</TableCell>
                  <TableCell className="text-right">
                    <div className="flex gap-1 justify-end">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleEditUsuario(usuario)}
                        className="text-gray-600 hover:text-gray-700"
                        title="Editar usuario"
                      >
                        <Edit className="h-3 w-3" />
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleDeleteUsuario(usuario)}
                        className="text-red-600 hover:text-red-700"
                        title="Eliminar usuario"
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
      {/* Total de usuarios */}
      <div className="mt-4 text-sm text-gray-600">
        Total: {usuarios.length} usuario{usuarios.length !== 1 ? "s" : ""}
      </div>
      {/* Dialog para agregar usuario */}
      <UsuarioFormDialog
        open={showAddForm}
        onOpenChange={setShowAddForm}
        onUsuarioAdded={handleUsuarioAdded}
      />
      {/* Dialog para editar usuario */}
      {editingUsuario && (
        <UsuarioFormDialog
          open={!!editingUsuario}
          onOpenChange={(open: boolean) => !open && setEditingUsuario(null)}
          onUsuarioAdded={handleUsuarioUpdated}
          usuario={editingUsuario}
          isEditing={true}
        />
      )}
    </div>
  );
}
