import { useAuth } from "@/contexts/AuthContext";

// Definición de roles y sus permisos
export type UserRole = "admin" | "tecnico" | "recepcion";

export interface RolePermissions {
  canManageUsers: boolean;
  canManageTermsConditions: boolean;
  canViewReports: boolean;
  canManageOrders: boolean;
  canManageClients: boolean;
  canManageEquipment: boolean;
  canManageParts: boolean;
}

// Configuración de permisos por rol
const rolePermissions: Record<UserRole, RolePermissions> = {
  admin: {
    canManageUsers: true,
    canManageTermsConditions: true,
    canViewReports: true,
    canManageOrders: true,
    canManageClients: true,
    canManageEquipment: true,
    canManageParts: true,
  },
  tecnico: {
    canManageUsers: false,
    canManageTermsConditions: false,
    canViewReports: true,
    canManageOrders: true,
    canManageClients: false,
    canManageEquipment: true,
    canManageParts: true,
  },
  recepcion: {
    canManageUsers: false,
    canManageTermsConditions: false,
    canViewReports: false,
    canManageOrders: true,
    canManageClients: true,
    canManageEquipment: true,
    canManageParts: false,
  },
};

/**
 * Hook para verificar permisos basado en el rol del usuario
 */
export function usePermissions() {
  const { user } = useAuth();

  const userRole = user?.usuario_rol as UserRole;
  const permissions = userRole ? rolePermissions[userRole] : null;

  // Función para verificar si el usuario tiene un permiso específico
  const hasPermission = (permission: keyof RolePermissions): boolean => {
    if (!permissions) return false;
    return permissions[permission];
  };

  // Función para verificar si el usuario tiene un rol específico
  const hasRole = (role: UserRole): boolean => {
    return userRole === role;
  };

  // Función para verificar si el usuario es admin
  const isAdmin = (): boolean => {
    return hasRole("admin");
  };

  // Función para verificar si el usuario es técnico
  const isTecnico = (): boolean => {
    return hasRole("tecnico");
  };

  // Función para verificar si el usuario es recepcionista
  const isRecepcion = (): boolean => {
    return hasRole("recepcion");
  };

  return {
    userRole,
    permissions,
    hasPermission,
    hasRole,
    isAdmin,
    isTecnico,
    isRecepcion,
  };
}

/**
 * Hook especializado para verificar acceso a vistas específicas
 */
export function useViewPermissions() {
  const { hasPermission, isAdmin } = usePermissions();

  return {
    canViewUsers: hasPermission("canManageUsers"),
    canViewTermsConditions: hasPermission("canManageTermsConditions"),
    canViewReports: hasPermission("canViewReports"),
    canViewOrders: hasPermission("canManageOrders"),
    canViewClients: hasPermission("canManageClients"),
    canViewEquipment: hasPermission("canManageEquipment"),
    canViewParts: hasPermission("canManageParts"),

    // Accesos específicos para vistas administrativas
    canAccessAdminViews: isAdmin(),
  };
}
