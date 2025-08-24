import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import {
  Database,
  AlertTriangle,
  RefreshCw,
  X,
  ChevronDown,
  Clock,
  Wifi,
  WifiOff,
  Server,
  AlertCircle,
} from "lucide-react";

interface DatabaseStatus {
  is_connected: boolean;
  error_message?: string;
  last_check?: string;
}

export function DatabaseConnectionBanner() {
  const [status, setStatus] = useState<DatabaseStatus | null>(null);
  const [isRetrying, setIsRetrying] = useState(false);
  const [isChecking, setIsChecking] = useState(false);
  const [isDismissed, setIsDismissed] = useState(false);
  const [showDetails, setShowDetails] = useState(false);
  const [connectionAttempts, setConnectionAttempts] = useState(0);

  const checkDatabaseStatus = async (showLoading = false) => {
    try {
      if (showLoading) setIsChecking(true);
      const result = await invoke<DatabaseStatus>("get_database_status");
      setStatus(result);

      // Si la conexión se restaura, mostrar el banner nuevamente y resetear intentos
      if (result.is_connected && isDismissed) {
        setIsDismissed(false);
        setConnectionAttempts(0);
      }
    } catch (error) {
      console.error("Error checking database status:", error);
      setStatus({
        is_connected: false,
        error_message: "Error al verificar el estado de la base de datos",
      });
    } finally {
      if (showLoading) setIsChecking(false);
    }
  };

  const retryConnection = async () => {
    try {
      setIsRetrying(true);
      setConnectionAttempts((prev) => prev + 1);
      const result = await invoke<DatabaseStatus>("retry_database_connection");
      setStatus(result);

      if (result.is_connected) {
        setIsDismissed(false);
        setConnectionAttempts(0);
        setShowDetails(false);
      }
    } catch (error) {
      console.error("Error retrying database connection:", error);
      setStatus((prev) => ({
        is_connected: false,
        error_message:
          "Error al intentar reconectar. Verifica la configuración de la base de datos.",
        last_check: prev?.last_check,
      }));
    } finally {
      setIsRetrying(false);
    }
  };

  const getErrorType = (errorMessage?: string) => {
    if (!errorMessage) return "unknown";

    const message = errorMessage.toLowerCase();
    if (
      message.includes("connection refused") ||
      message.includes("denegó expresamente")
    ) {
      return "connection_refused";
    }
    if (message.includes("timeout") || message.includes("time out")) {
      return "timeout";
    }
    if (
      message.includes("authentication") ||
      message.includes("access denied")
    ) {
      return "auth";
    }
    if (
      message.includes("database") &&
      message.includes("not") &&
      message.includes("exist")
    ) {
      return "database_not_exist";
    }
    return "unknown";
  };

  const getErrorSolution = (errorType: string) => {
    switch (errorType) {
      case "connection_refused":
        return "Verifica que el servidor MySQL esté ejecutándose y accesible en el puerto configurado.";
      case "timeout":
        return "El servidor responde lentamente. Verifica la conexión de red y la configuración del servidor.";
      case "auth":
        return "Credenciales incorrectas. Verifica el usuario y contraseña en la configuración.";
      case "database_not_exist":
        return "La base de datos especificada no existe. Créala o verifica el nombre en la configuración.";
      default:
        return "Verifica la configuración de conexión en el archivo .env y que el servidor MySQL esté disponible.";
    }
  };
  useEffect(() => {
    checkDatabaseStatus();
    // Verificar el estado cada 30 segundos
    const interval = setInterval(() => checkDatabaseStatus(false), 30000);
    return () => clearInterval(interval);
  }, []);

  // No mostrar nada si está conectado, no hay estado, o fue descartado
  if (!status || status.is_connected || isDismissed) {
    return null;
  }

  const errorType = getErrorType(status.error_message);
  const solution = getErrorSolution(errorType);
  const lastCheck = status.last_check ? new Date(status.last_check) : null;

  return (
    <div className="w-full p-4 border-b bg-red-50 dark:bg-red-950/20">
      <Card className="border-red-200 dark:border-red-800">
        <CardContent className="p-4">
          <div className="flex items-start gap-4">
            {/* Icono de estado */}
            <div className="flex-shrink-0 mt-1">
              <div className="relative">
                <Server className="h-6 w-6 text-red-600 dark:text-red-400" />
                <div className="absolute -bottom-1 -right-1">
                  <WifiOff className="h-3 w-3 text-red-500 bg-white dark:bg-gray-900 rounded-full" />
                </div>
              </div>
            </div>

            {/* Contenido principal */}
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2 mb-2">
                <AlertTriangle className="h-4 w-4 text-red-600 dark:text-red-400" />
                <h3 className="font-semibold text-red-900 dark:text-red-100">
                  Base de Datos Desconectada
                </h3>
                <Badge variant="destructive" className="text-xs">
                  {connectionAttempts > 0
                    ? `${connectionAttempts} intentos`
                    : "Sin conexión"}
                </Badge>
              </div>
              <p className="text-sm text-red-800 dark:text-red-200 mb-3">
                No se puede establecer conexión con la base de datos. Algunas
                funcionalidades pueden no estar disponibles.
              </p>{" "}
              {/* Detalles expandibles */}
              <div className="space-y-2">
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setShowDetails(!showDetails)}
                  className="p-0 h-auto text-red-700 dark:text-red-300 hover:text-red-900 dark:hover:text-red-100"
                >
                  <ChevronDown
                    className={`h-4 w-4 mr-1 transition-transform ${
                      showDetails ? "rotate-180" : ""
                    }`}
                  />
                  Ver detalles técnicos
                </Button>

                {showDetails && (
                  <div className="mt-3 bg-red-100 dark:bg-red-900/30 rounded-lg p-3 space-y-2">
                    <div className="flex items-center gap-2 text-xs text-red-700 dark:text-red-300">
                      <AlertCircle className="h-3 w-3" />
                      <span className="font-medium">Error:</span>
                      <code className="bg-red-200 dark:bg-red-800 px-1 rounded text-xs">
                        {status.error_message || "Error desconocido"}
                      </code>
                    </div>

                    {lastCheck && (
                      <div className="flex items-center gap-2 text-xs text-red-600 dark:text-red-400">
                        <Clock className="h-3 w-3" />
                        <span>
                          Última verificación: {lastCheck.toLocaleString()}
                        </span>
                      </div>
                    )}

                    <div className="mt-2 p-2 bg-red-50 dark:bg-red-900/50 rounded border border-red-200 dark:border-red-800">
                      <p className="text-xs text-red-800 dark:text-red-200">
                        <strong>Solución sugerida:</strong> {solution}
                      </p>
                    </div>
                  </div>
                )}
              </div>
            </div>

            {/* Acciones */}
            <div className="flex-shrink-0 flex items-center gap-2">
              <Button
                onClick={() => checkDatabaseStatus(true)}
                disabled={isChecking}
                variant="outline"
                size="sm"
                className="border-red-300 text-red-700 hover:bg-red-100 dark:border-red-700 dark:text-red-300 dark:hover:bg-red-900/30"
              >
                {isChecking ? (
                  <RefreshCw className="h-3 w-3 mr-1 animate-spin" />
                ) : (
                  <Wifi className="h-3 w-3 mr-1" />
                )}
                Verificar
              </Button>

              <Button
                onClick={retryConnection}
                disabled={isRetrying}
                variant="default"
                size="sm"
                className="bg-red-600 hover:bg-red-700 text-white"
              >
                {isRetrying ? (
                  <RefreshCw className="h-3 w-3 mr-1 animate-spin" />
                ) : (
                  <Database className="h-3 w-3 mr-1" />
                )}
                Reconectar
              </Button>

              <Button
                onClick={() => setIsDismissed(true)}
                variant="ghost"
                size="sm"
                className="text-red-600 hover:text-red-800 hover:bg-red-100 dark:text-red-400 dark:hover:text-red-200 dark:hover:bg-red-900/30 p-2"
              >
                <X className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
