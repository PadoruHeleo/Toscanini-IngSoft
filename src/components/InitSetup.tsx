import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";

interface Usuario {
  usuario_id: number;
  usuario_rut: string | null;
  usuario_nombre: string | null;
  usuario_correo: string | null;
  usuario_contrasena: string | null;
  usuario_telefono: string | null;
  usuario_rol: string | null;
}

export function InitSetup() {
  const [isCreating, setIsCreating] = useState(false);
  const [message, setMessage] = useState("");

  const createAdminUser = async () => {
    setIsCreating(true);
    setMessage("");
    try {
      const result = await invoke<Usuario>("create_admin_user");

      // Determinar si es un usuario existente o recién creado
      const isNewUser =
        !result.usuario_id || result.usuario_correo === "admin@toscanini.com";

      let statusMessage = "";
      if (isNewUser) {
        statusMessage = "✅ Usuario administrador creado exitosamente.\n";
      } else {
        statusMessage = "✅ Usuario administrador encontrado.\n";
      }

      statusMessage += `👤 Nombre: ${result.usuario_nombre || "N/A"}\n`;
      statusMessage += `📧 Email: ${result.usuario_correo || "N/A"}\n`;
      statusMessage += `📱 Teléfono: ${result.usuario_telefono || "N/A"}\n`;
      statusMessage += `🔐 Rol: ${result.usuario_rol || "N/A"}`;

      // Solo mostrar contraseña por defecto si es el usuario admin por defecto
      if (result.usuario_correo === "admin@toscanini.com") {
        statusMessage += "\n🔑 Contraseña por defecto: admin123";
      }

      setMessage(statusMessage);
    } catch (error) {
      const errorString = String(error);
      setMessage(`❌ Error: ${errorString}`);
    } finally {
      setIsCreating(false);
    }
  };

  return (
    <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg mb-4">
      <h3 className="text-sm font-medium text-blue-800 mb-2">
        Configuración Inicial
      </h3>
      <p className="text-sm text-blue-600 mb-3">
        ¿Es la primera vez que usas el sistema? Crea un usuario administrador
        para comenzar.
      </p>
      <Button
        onClick={createAdminUser}
        disabled={isCreating}
        size="sm"
        variant="outline"
      >
        {isCreating ? "Creando..." : "Crear Usuario Admin"}
      </Button>
      {message && (
        <div className="text-sm mt-2 text-blue-700 whitespace-pre-line bg-white p-2 rounded border">
          {message}
        </div>
      )}
    </div>
  );
}
