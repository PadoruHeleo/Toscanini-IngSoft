import { useAuth } from "@/contexts/AuthContext";

export function SessionInfo() {
  const { user } = useAuth();

  if (!user?.session_expires_at) return null;

  const expirationTime = new Date(user.session_expires_at);
  const now = new Date();
  const difference = expirationTime.getTime() - now.getTime();

  if (difference <= 0) return null; // Sesión expirada

  const isExpiringSoon = difference < 30 * 60 * 1000; // 30 minutos
  const lastLogin = user.last_login_at
    ? new Date(user.last_login_at).toLocaleString()
    : "N/A";

  return (
    <div className="flex items-center gap-2 text-xs text-muted-foreground">
      <div
        className={`w-2 h-2 rounded-full ${
          isExpiringSoon ? "bg-amber-500" : "bg-green-500"
        }`}
        title={`Sesión activa • Último acceso: ${lastLogin}`}
      />
      <span className="truncate">
        {isExpiringSoon ? "Sesión por expirar" : "Sesión activa"}
      </span>
    </div>
  );
}
