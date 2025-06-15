import { useEffect, useState } from "react";
import { useAuth } from "@/contexts/AuthContext";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";

export function SessionExpirationWarning() {
  const { user, logout, validateSession } = useAuth();
  const [showWarning, setShowWarning] = useState(false);
  const [timeLeft, setTimeLeft] = useState<number>(0);

  useEffect(() => {
    if (!user?.session_expires_at) return;

    const interval = setInterval(() => {
      const expirationTime = new Date(user.session_expires_at!);
      const now = new Date();
      const difference = expirationTime.getTime() - now.getTime();

      // Mostrar advertencia cuando quedan 5 minutos (300000 ms)
      if (difference > 0 && difference <= 300000 && !showWarning) {
        setShowWarning(true);
        setTimeLeft(Math.ceil(difference / 1000 / 60)); // Minutos restantes
      } else if (difference <= 0) {
        // Sesión expirada
        setShowWarning(false);
        logout();
      }
    }, 30000); // Verificar cada 30 segundos

    return () => clearInterval(interval);
  }, [user?.session_expires_at, showWarning, logout]);

  const handleContinueSession = async () => {
    // Validar que la sesión aún esté activa en el servidor
    const isValid = await validateSession();
    if (isValid) {
      setShowWarning(false);
    } else {
      logout();
    }
  };

  const handleLogout = () => {
    setShowWarning(false);
    logout();
  };

  return (
    <Dialog open={showWarning} onOpenChange={setShowWarning}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Sesión por expirar</DialogTitle>
          <DialogDescription>
            Tu sesión expirará en aproximadamente {timeLeft} minuto
            {timeLeft !== 1 ? "s" : ""}. ¿Deseas continuar trabajando?
          </DialogDescription>
        </DialogHeader>
        <DialogFooter className="flex-col sm:flex-row gap-2">
          <Button variant="outline" onClick={handleLogout}>
            Cerrar sesión
          </Button>
          <Button onClick={handleContinueSession}>Continuar sesión</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
