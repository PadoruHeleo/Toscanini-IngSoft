import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { CheckCircle2, XCircle, Loader2 } from "lucide-react";

interface DatabaseStatus {
  is_connected: boolean;
  error_message?: string;
  last_check?: string;
}

export function DatabaseStatusIndicator() {
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
      <div className="fixed bottom-4 right-4 z-50 flex items-center gap-2 px-2 py-1.5 bg-gray-100 border border-gray-300 rounded-md shadow-sm">
        <Loader2 className="h-3 w-3 animate-spin text-gray-500" />
        <span className="text-xs text-gray-600">Verificando...</span>
      </div>
    );
  }

  return (
    <div
      className={`fixed bottom-4 right-4 z-50 flex items-center gap-2 px-2 py-1.5 rounded-md shadow-sm border transition-all ${
        status.is_connected
          ? "bg-green-50 border-green-200 hover:bg-green-100"
          : "bg-red-50 border-red-200 hover:bg-red-100"
      }`}
      title={
        status.is_connected
          ? "Base de datos conectada"
          : status.error_message || "Base de datos desconectada"
      }
    >
      {isChecking ? (
        <Loader2 className="h-3 w-3 animate-spin text-gray-500" />
      ) : status.is_connected ? (
        <CheckCircle2 className="h-3 w-3 text-green-600" />
      ) : (
        <XCircle className="h-3 w-3 text-red-600" />
      )}
      <span
        className={`text-xs font-medium ${
          status.is_connected ? "text-green-700" : "text-red-700"
        }`}
      >
        DB
      </span>
      {!status.is_connected && (
        <div className="h-2 w-2 rounded-full bg-red-500 animate-pulse" />
      )}
    </div>
  );
}

