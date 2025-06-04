import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";

export function InitSetup() {
  const [isCreating, setIsCreating] = useState(false);
  const [message, setMessage] = useState("");
  const createAdminUser = async () => {
    setIsCreating(true);
    setMessage("");
    try {
      await invoke("create_admin_user");
      setMessage(
        "✅ Usuario administrador disponible.\n📧 Email: admin@toscanini.com\n🔑 Contraseña: admin123"
      );
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
      </Button>{" "}
      {message && (
        <div className="text-sm mt-2 text-blue-700 whitespace-pre-line bg-white p-2 rounded border">
          {message}
        </div>
      )}
    </div>
  );
}
