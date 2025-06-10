import React, { createContext, useContext, useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface User {
  usuario_id: number;
  usuario_rut: string | null;
  usuario_nombre: string | null;
  usuario_correo: string | null;
  usuario_telefono: string | null;
  usuario_rol: string | null;
  last_login_at: string | null;
  session_expires_at: string | null;
  session_token: string | null;
}

interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (
    email: string,
    password: string
  ) => Promise<{ success: boolean; error?: string }>;
  logout: () => void;
  validateSession: () => Promise<boolean>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const isAuthenticated = user !== null;

  // Función para verificar si la sesión ha expirado
  const isSessionExpired = (sessionExpiresAt: string | null): boolean => {
    if (!sessionExpiresAt) return true;
    return new Date(sessionExpiresAt) <= new Date();
  };

  // Función para validar la sesión actual
  const validateSession = async (): Promise<boolean> => {
    const savedUser = localStorage.getItem("user");
    if (!savedUser) return false;

    try {
      const userData = JSON.parse(savedUser) as User;

      // Verificar si la sesión ha expirado localmente
      if (isSessionExpired(userData.session_expires_at)) {
        localStorage.removeItem("user");
        setUser(null);
        return false;
      }

      // Validar con el backend si el token sigue siendo válido
      if (userData.session_token) {
        const result = await invoke<User | null>("validate_session", {
          sessionToken: userData.session_token,
        });

        if (result) {
          setUser(result);
          localStorage.setItem("user", JSON.stringify(result));
          return true;
        } else {
          localStorage.removeItem("user");
          setUser(null);
          return false;
        }
      }
    } catch (error) {
      console.error("Error validating session:", error);
      localStorage.removeItem("user");
      setUser(null);
    }

    return false;
  };

  useEffect(() => {
    // Verificar si hay una sesión guardada al cargar la app
    const initializeAuth = async () => {
      await validateSession();
      setIsLoading(false);
    };

    initializeAuth();
  }, []);

  // Configurar un intervalo para verificar la expiración de sesión cada minuto
  useEffect(() => {
    if (!user) return;

    const interval = setInterval(() => {
      if (
        user.session_expires_at &&
        isSessionExpired(user.session_expires_at)
      ) {
        console.log("Sesión expirada, cerrando sesión automáticamente");
        logout();
      }
    }, 60000); // Verificar cada minuto

    return () => clearInterval(interval);
  }, [user]);
  const login = async (
    email: string,
    password: string
  ): Promise<{ success: boolean; error?: string }> => {
    try {
      // NO activamos setIsLoading aquí para evitar la pantalla de carga completa
      const result = await invoke<User | null>("authenticate_usuario", {
        usuarioCorreo: email,
        usuarioContrasena: password,
      });

      if (result) {
        setUser(result);
        localStorage.setItem("user", JSON.stringify(result));
        return { success: true };
      }
      return { success: false, error: "UNKNOWN_ERROR" };
    } catch (error) {
      console.error("Login error:", error);
      return { success: false, error: String(error) };
    }
    // NO hay finally con setIsLoading(false) aquí
  };

  const logout = async () => {
    // Si hay un token de sesión, notificar al backend
    if (user?.session_token) {
      try {
        await invoke("logout_user", {
          sessionToken: user.session_token,
        });
      } catch (error) {
        console.error("Error during logout:", error);
      }
    }

    setUser(null);
    localStorage.removeItem("user");
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        isAuthenticated,
        isLoading,
        login,
        logout,
        validateSession,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
