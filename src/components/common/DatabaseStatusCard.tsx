import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { CheckCircle2, XCircle, Loader2 } from "lucide-react";

interface DatabaseStatus {
  is_connected: boolean;
  error_message?: string;
  last_check?: string;
}

export function DatabaseStatusCard() {
  const [status, setStatus] = useState<DatabaseStatus | null>(null);
  const [isChecking, setIsChecking] = useState(false);

  const checkDatabaseStatus = async () => {
    try {
      setIsChecking(true);
      const result = await invoke<DatabaseStatus>("get_database_status");
      setStatus(result);
    } catch (error) {
      console.error("Error checking database status:", error);
      setStatus({
        is_connected: false,
        error_message: "Error al verificar el estado de la base de datos",
      });
    } finally {
      setIsChecking(false);
    }
  };

  useEffect(() => {
    checkDatabaseStatus();
    // Verificar el estado cada 30 segundos
    const interval = setInterval(checkDatabaseStatus, 30000);
    return () => clearInterval(interval);
  }, []);

  if (!status) {
    return (
      <div className="p-3 bg-gray-50 border border-gray-200 rounded-lg mb-4 flex items-center gap-2">
        <Loader2 className="h-4 w-4 animate-spin text-gray-500" />
        <span className="text-sm text-gray-600">Verificando conexión...</span>
      </div>
    );
  }

  return (
    <div
      className={`p-3 border rounded-lg mb-4 flex items-center justify-between ${
        status.is_connected
          ? "bg-green-50 border-green-200"
          : "bg-red-50 border-red-200"
      }`}
    >
      <div className="flex items-center gap-2">
        {status.is_connected ? (
          <CheckCircle2 className="h-4 w-4 text-green-600" />
        ) : (
          <XCircle className="h-4 w-4 text-red-600" />
        )}
        <div>
          <p
            className={`text-sm font-medium ${
              status.is_connected ? "text-green-800" : "text-red-800"
            }`}
          >
            Base de Datos: {status.is_connected ? "Conectada" : "Desconectada"}
          </p>
          {!status.is_connected && status.error_message && (
            <p className="text-xs text-red-600 mt-1">
              {status.error_message.length > 50
                ? `${status.error_message.substring(0, 50)}...`
                : status.error_message}
            </p>
          )}
        </div>
      </div>
      {isChecking && (
        <Loader2 className="h-4 w-4 animate-spin text-gray-500" />
      )}
    </div>
  );
}

