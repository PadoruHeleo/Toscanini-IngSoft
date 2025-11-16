import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Alert, AlertDescription } from "@/components/ui/alert";
import {
  RefreshCw,
  Database,
  AlertCircle,
  CheckCircle,
  Wifi,
  WifiOff,
} from "lucide-react";

interface DatabaseStatus {
  is_connected: boolean;
  error_message?: string;
  last_check?: string;
}

export function DatabaseConnectionStatus() {
  const [status, setStatus] = useState<DatabaseStatus | null>(null);
  const [isChecking, setIsChecking] = useState(false);
  const [isRetrying, setIsRetrying] = useState(false);

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

  const [debugMessage, setDebugMessage] = useState<string>("");
  const [isDebugging, setIsDebugging] = useState(false);

  const checkDataCounts = async () => {
    try {
      setIsDebugging(true);
      const result = await invoke<string>("insert_test_data");
      setDebugMessage(result);
    } catch (error) {
      setDebugMessage(`Error: ${error}`);
    } finally {
      setIsDebugging(false);
    }
  };

  const checkEquipoIds = async () => {
    try {
      setIsDebugging(true);
      const result = await invoke<string>("check_equipo_ids");
      setDebugMessage(result);
    } catch (error) {
      setDebugMessage(`Error: ${error}`);
    } finally {
      setIsDebugging(false);
    }
  };

  const runMigrations = async () => {
    try {
      setIsDebugging(true);
      const result = await invoke<string>("force_run_migrations");
      setDebugMessage(result);
    } catch (error) {
      setDebugMessage(`Error: ${error}`);
    } finally {
      setIsDebugging(false);
    }
  };

  const retryConnection = async () => {
    try {
      setIsRetrying(true);
      const result = await invoke<DatabaseStatus>("retry_database_connection");
      setStatus(result);
    } catch (error) {
      console.error("Error retrying database connection:", error);
      setStatus({
        is_connected: false,
        error_message: "Error al intentar reconectar a la base de datos",
      });
    } finally {
      setIsRetrying(false);
    }
  };

  const recheckConnection = async () => {
    try {
      setIsChecking(true);
      const result = await invoke<DatabaseStatus>("check_database_connection");
      setStatus(result);
    } catch (error) {
      console.error("Error rechecking database connection:", error);
      setStatus({
        is_connected: false,
        error_message: "Error al verificar la conexión a la base de datos",
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
      <Card className="w-full max-w-md mx-auto">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Database className="h-5 w-5" />
            Estado de la Base de Datos
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-center py-4">
            <RefreshCw className="h-6 w-6 animate-spin" />
            <span className="ml-2">Verificando estado...</span>
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className="w-full max-w-md mx-auto">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Database className="h-5 w-5" />
          Estado de la Base de Datos
        </CardTitle>
        <CardDescription>
          {status.last_check && (
            <>
              Última verificación:{" "}
              {new Date(status.last_check).toLocaleString()}
            </>
          )}
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex items-center gap-3">
          {status.is_connected ? (
            <>
              <div className="flex items-center gap-2 text-green-600">
                <CheckCircle className="h-5 w-5" />
                <Wifi className="h-4 w-4" />
              </div>
              <span className="text-green-600 font-medium">Conectado</span>
            </>
          ) : (
            <>
              <div className="flex items-center gap-2 text-red-600">
                <AlertCircle className="h-5 w-5" />
                <WifiOff className="h-4 w-4" />
              </div>
              <span className="text-red-600 font-medium">Desconectado</span>
            </>
          )}
        </div>

        {!status.is_connected && status.error_message && (
          <Alert variant="destructive">
            <AlertCircle className="h-4 w-4" />
            <AlertDescription>
              <strong>Error de conexión:</strong>
              <br />
              {status.error_message}
            </AlertDescription>
          </Alert>
        )}

        {debugMessage && (
          <Alert>
            <AlertDescription>
              <pre className="whitespace-pre-wrap text-xs">{debugMessage}</pre>
            </AlertDescription>
          </Alert>
        )}

        <div className="space-y-2">
          <div className="flex gap-2">
            <Button
              onClick={recheckConnection}
              disabled={isChecking}
              variant="outline"
              size="sm"
              className="flex-1"
            >
              {isChecking ? (
                <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
              ) : (
                <RefreshCw className="h-4 w-4 mr-2" />
              )}
              Verificar
            </Button>

            {!status.is_connected && (
              <Button
                onClick={retryConnection}
                disabled={isRetrying}
                variant="default"
                size="sm"
                className="flex-1"
              >
                {isRetrying ? (
                  <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
                ) : (
                  <Database className="h-4 w-4 mr-2" />
                )}
                Reconectar
              </Button>
            )}
          </div>

          <div className="flex gap-2">
            <Button
              onClick={runMigrations}
              disabled={isDebugging}
              variant="secondary"
              size="sm"
              className="flex-1"
            >
              {isDebugging ? (
                <RefreshCw className="h-4 w-4 mr-2 animate-spin" />
              ) : (
                <Database className="h-4 w-4 mr-2" />
              )}
              Ejecutar Migraciones
            </Button>

            <Button
              onClick={checkDataCounts}
              disabled={isDebugging}
              variant="secondary"
              size="sm"
              className="flex-1"
            >
              Conteos
            </Button>
          </div>

          <Button
            onClick={checkEquipoIds}
            disabled={isDebugging}
            variant="secondary"
            size="sm"
            className="w-full"
          >
            Ver IDs Equipos
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
