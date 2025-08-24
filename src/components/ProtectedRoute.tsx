import React from "react";
import { useAuth } from "@/contexts/AuthContext";
import { useView } from "@/contexts/ViewContext";
import { LoginView } from "@/components/views/LoginView";
import { PasswordResetView } from "@/components/views/PasswordResetView";

interface ProtectedRouteProps {
  children: React.ReactNode;
  fallback?: React.ReactNode;
}

export function ProtectedRoute({ children, fallback }: ProtectedRouteProps) {
  const { isAuthenticated, isLoading } = useAuth();
  const { currentView } = useView();

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 mx-auto mb-4"></div>
          <p>Cargando...</p>
        </div>
      </div>
    );
  }
  if (!isAuthenticated) {
    // Handle non-authenticated views
    if (currentView === "password-reset") {
      return <PasswordResetView />;
    }
    // Default to login view for any other case when not authenticated
    return fallback || <LoginView />;
  }

  return <>{children}</>;
}
