import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useAuth } from "@/contexts/AuthContext";
import { useToastContext } from "@/contexts/ToastContext";

interface Usuario {
  usuario_id: number;
  usuario_rut?: string;
  usuario_nombre?: string;
  usuario_correo?: string;
  usuario_telefono?: string;
  usuario_rol?: string;
  created_by?: number;
  created_at?: string;
}

interface CreateUsuarioRequest {
  usuario_rut: string;
  usuario_nombre: string;
  usuario_correo: string;
  usuario_telefono?: string;
  usuario_rol: string;
  usuario_contrasena: string;
  created_by: number;
}

interface UpdateUsuarioRequest {
  usuario_rut?: string;
  usuario_nombre?: string;
  usuario_correo?: string;
  usuario_telefono?: string;
  usuario_rol?: string;
}

interface UsuarioFormDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onUsuarioAdded: () => void;
  usuario?: Usuario;
  isEditing?: boolean;
}

export function UsuarioFormDialog({
  open,
  onOpenChange,
  onUsuarioAdded,
  usuario,
  isEditing = false,
}: UsuarioFormDialogProps) {
  const { user } = useAuth();
  const { success, error: showError } = useToastContext();
  const [loading, setLoading] = useState(false);
  const [showConfirmationDialog, setShowConfirmationDialog] = useState(false);
  const [formData, setFormData] = useState({
    usuario_rut: "",
    usuario_nombre: "",
    usuario_correo: "",
    usuario_telefono: "",
    usuario_rol: "",
  });
  const [errors, setErrors] = useState<Record<string, string>>({});

  useEffect(() => {
    if (isEditing && usuario) {
      setFormData({
        usuario_rut: usuario.usuario_rut || "",
        usuario_nombre: usuario.usuario_nombre || "",
        usuario_correo: usuario.usuario_correo || "",
        usuario_telefono: usuario.usuario_telefono || "",
        usuario_rol: usuario.usuario_rol || "",
      });
    } else {
      setFormData({
        usuario_rut: "",
        usuario_nombre: "",
        usuario_correo: "",
        usuario_telefono: "",
        usuario_rol: "",
      });
    }
    setErrors({});
  }, [isEditing, usuario, open]);

  const correoYaExiste = async (correo: string): Promise<boolean> => {
    try {
      const existe = await invoke<boolean>("verify_email_in_use", {
        correo,
      });
      return existe;
    } catch (error) {
      console.error("Error verificando correo:", error);
      return false;
    }
  };

  const rutYaExiste = async (rut: string): Promise<boolean> => {
    try {
      return await invoke<boolean>("verify_rut_in_use", { rut });
    } catch (error) {
      console.error("Error verificando RUT:", error);
      return false;
    }
  };

  const validateForm = async (): Promise<boolean> => {
  const newErrors: Record<string, string> = {};

    // Validación de teléfono usando función Tauri
    if (formData.usuario_telefono) {
      const telefonoValido = await invoke<boolean>("verify_phone", { phone: formData.usuario_telefono });
      if (!telefonoValido) {
        newErrors.usuario_telefono = "El teléfono debe tener formato +569XXXXXXXX";
      }
    }

    if (!formData.usuario_rut.trim()) {
      newErrors.usuario_rut = "El RUT es obligatorio";
    } else if (
      // Solo valida si es creación o si el RUT cambió
      (!isEditing || formData.usuario_rut !== usuario?.usuario_rut) &&
      (await rutYaExiste(formData.usuario_rut))
    ) {
      newErrors.usuario_rut = "Este RUT ya está registrado";
    }

    if (!formData.usuario_nombre.trim()) {
      newErrors.usuario_nombre = "El nombre es obligatorio";
    }
    if (!formData.usuario_correo.trim()) {
      newErrors.usuario_correo = "El correo es obligatorio";
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.usuario_correo)) {
      newErrors.usuario_correo = "El formato del correo no es válido";
    } else if (
      (!isEditing || formData.usuario_correo !== usuario?.usuario_correo) &&
      (await correoYaExiste(formData.usuario_correo))
    ) {
      newErrors.usuario_correo = "Este correo ya está registrado";
    }
    if (!formData.usuario_rol.trim()) {
      newErrors.usuario_rol = "El rol es obligatorio";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!(await validateForm())) {
      return;
    }
    setShowConfirmationDialog(true);
  };

  const handleConfirmSubmit = async () => {
    if (!user) {
      showError("Error de autenticación", "Usuario no autenticado");
      setShowConfirmationDialog(false);
      return;
    }


    try {
      setLoading(true);

      if (isEditing && usuario) {
        const updateRequest: UpdateUsuarioRequest = {
          usuario_rut: formData.usuario_rut || undefined,
          usuario_nombre: formData.usuario_nombre || undefined,
          usuario_correo: formData.usuario_correo || undefined,
          usuario_telefono: formData.usuario_telefono || undefined,
          usuario_rol: formData.usuario_rol || undefined,
        };

        const updatedUsuario = await invoke<Usuario>("update_usuario", {
          usuarioId: usuario.usuario_id,
          request: updateRequest,
          updatedBy: user?.usuario_id || 0,
        });

        if (updatedUsuario) {
          success(
            "¡Usuario actualizado exitosamente!",
            `${formData.usuario_nombre} ha sido actualizado correctamente.`
          );
        } else {
          showError("Error", "No se pudo actualizar el usuario.");
          return;
        }
      } else {
        // 1. Genera la contraseña temporal
        const tempPassword = generarContrasena();

        // 2. Crea el usuario en el backend
        const createRequest: CreateUsuarioRequest = {
          usuario_rut: formData.usuario_rut,
          usuario_nombre: formData.usuario_nombre,
          usuario_correo: formData.usuario_correo,
          usuario_telefono: formData.usuario_telefono || undefined,
          usuario_rol: formData.usuario_rol,
          usuario_contrasena: tempPassword,
          created_by: user?.usuario_id || 0,
        };

        await invoke<Usuario>("create_usuario", {
          request: createRequest,
        });

        // 3. Envía el email con la contraseña temporal
        await invoke("send_password_email", {
          toEmail: formData.usuario_correo,
          userName: formData.usuario_nombre,
          tempPassword: tempPassword,
        });

        success(
          "¡Usuario creado exitosamente!",
          `${formData.usuario_nombre} ha sido registrado correctamente. Se envió la contraseña por correo.`
        );
      }

      onOpenChange(false);
      setFormData({
        usuario_rut: "",
        usuario_nombre: "",
        usuario_correo: "",
        usuario_telefono: "",
        usuario_rol: "",
      });
      setErrors({});
      onUsuarioAdded();
    } catch (error) {
      console.error("Error procesando usuario:", error);

      showError(
        isEditing ? "Error al actualizar usuario" : "Error al crear usuario",
        typeof error === "string"
          ? error
          : "Ha ocurrido un error inesperado. Por favor, intente nuevamente."
      );
      setErrors({
        submit: `Error al ${
          isEditing ? "actualizar" : "crear"
        } el usuario. Intente nuevamente.`,
      });
    } finally {
      setLoading(false);
      setShowConfirmationDialog(false);
    }
  };

  const handleInputChange = (field: keyof typeof formData, value: string) => {
    if (field === "usuario_rut") {
      // Limita a máximo 9 caracteres antes de formatear
      value = value.replace(/[.\-]/g, "").slice(0, 9);
      value = formatRut(value);
    }
    setFormData((prev) => ({ ...prev, [field]: value }));
    if (errors[field]) {
      setErrors((prev) => ({ ...prev, [field]: "" }));
    }
  };

  function generarContrasena(longitud = 10) {
    const chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%&";
    let contrasena = "";
    for (let i = 0; i < longitud; i++) {
      contrasena += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return contrasena;
  }

  function formatRut(rut: string) {
    rut = rut.replace(/[^\dkK]/gi, ""); // Elimina todo menos números y k/K
    if (rut.length <= 1) return rut;
    let cuerpo = rut.slice(0, -1);
    let dv = rut.slice(-1);
    cuerpo = cuerpo.replace(/\B(?=(\d{3})+(?!\d))/g, ".");
    return `${cuerpo}-${dv}`;
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>
            {isEditing ? "Editar Usuario" : "Agregar Nuevo Usuario"}
          </DialogTitle>
          <DialogDescription>
            {isEditing
              ? "Modifique la información del usuario."
              : "Complete la información del usuario que desea registrar."}
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="usuario_rut">RUT *</Label>
            <Input
              id="usuario_rut"
              value={formData.usuario_rut}
              onChange={(e) => handleInputChange("usuario_rut", e.target.value)}
              placeholder="Ej: 12.345.678-9"
              className={errors.usuario_rut ? "border-red-500" : ""}
              maxLength={12} // 9 dígitos + puntos y guion
            />
            {errors.usuario_rut && (
              <p className="text-sm text-red-500">{errors.usuario_rut}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="usuario_nombre">Nombre *</Label>
            <Input
              id="usuario_nombre"
              value={formData.usuario_nombre}
              onChange={(e) =>
                handleInputChange("usuario_nombre", e.target.value)
              }
              placeholder="Nombre completo del usuario"
              className={errors.usuario_nombre ? "border-red-500" : ""}
            />
            {errors.usuario_nombre && (
              <p className="text-sm text-red-500">{errors.usuario_nombre}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="usuario_correo">Correo Electrónico *</Label>
            <Input
              id="usuario_correo"
              type="email"
              value={formData.usuario_correo}
              onChange={(e) =>
                handleInputChange("usuario_correo", e.target.value)
              }
              placeholder="usuario@ejemplo.com"
              className={errors.usuario_correo ? "border-red-500" : ""}
            />
            {errors.usuario_correo && (
              <p className="text-sm text-red-500">{errors.usuario_correo}</p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="usuario_telefono">Teléfono</Label>
            <Input
              id="usuario_telefono"
              value={formData.usuario_telefono}
              onChange={(e) => {
                let value = e.target.value.replace(/[^\d+]/g, "");
                value = value.slice(0, 12); // Limita a 12 caracteres (+569XXXXXXXX)
                handleInputChange("usuario_telefono", value);
              }}
              className={`px-3 py-2 text-base w-full${errors.usuario_telefono ? " border-red-500" : ""}`}
              placeholder="+56912345678"
              maxLength={12}
              style={{ minWidth: "180px" }}
            />
            {errors.usuario_telefono && (
              <div className="w-full">
                <p className="text-sm text-red-500 mt-2">{errors.usuario_telefono}</p>
              </div>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="usuario_rol">Rol *</Label>
            <select
              id="usuario_rol"
              value={formData.usuario_rol}
              onChange={(e) => handleInputChange("usuario_rol", e.target.value)}
              className={`w-full border rounded px-3 py-2 ${errors.usuario_rol ? "border-red-500" : ""}`}
            >
              <option value="">Seleccione un rol</option>
              <option value="recepcion">Recepcionista</option>
              <option value="tecnico">Técnico</option>
              <option value="admin">Administrador</option>
            </select>
            {errors.usuario_rol && (
              <p className="text-sm text-red-500">{errors.usuario_rol}</p>
            )}
          </div>
          {errors.submit && (
            <div className="text-sm text-red-500 bg-red-50 p-3 rounded-md">
              {errors.submit}
            </div>
          )}
          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button type="submit" disabled={loading}>
              {loading
                ? isEditing
                  ? "Actualizando..."
                  : "Creando..."
                : isEditing
                ? "Actualizar Usuario"
                : "Crear Usuario"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>

      {/* Modal de confirmación */}
      <Dialog
        open={showConfirmationDialog}
        onOpenChange={setShowConfirmationDialog}
      >
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>
              {isEditing
                ? "Confirmar Actualización"
                : "Confirmar Creación de Usuario"}
            </DialogTitle>
            <DialogDescription>
              {isEditing
                ? "¿Está seguro que desea actualizar este usuario con los cambios realizados?"
                : "¿Está seguro que desea crear este usuario con la siguiente información?"}
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-2 text-sm">
            <div>
              <strong>RUT:</strong> {formData.usuario_rut}
            </div>
            <div>
              <strong>Nombre:</strong> {formData.usuario_nombre}
            </div>
            <div>
              <strong>Correo:</strong> {formData.usuario_correo}
            </div>
            {formData.usuario_telefono && (
              <div>
                <strong>Teléfono:</strong> {formData.usuario_telefono}
              </div>
            )}
            <div>
              <strong>Rol:</strong> {formData.usuario_rol}
            </div>
          </div>

          <DialogFooter className="gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setShowConfirmationDialog(false)}
              disabled={loading}
            >
              Cancelar
            </Button>
            <Button onClick={handleConfirmSubmit} disabled={loading}>
              {loading
                ? "Procesando..."
                : isEditing
                ? "Confirmar Actualización"
                : "Confirmar Creación"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Dialog>
  );
}