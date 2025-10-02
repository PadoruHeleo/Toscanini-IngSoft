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

/**
 * Hook especializado para permisos específicos de gestión de clientes
 */
export function useClientePermissions() {
  const { userRole, isAdmin, isRecepcion } = usePermissions();

  return {
    // Crear cliente: solo admin y recepción
    canCreateCliente: isAdmin() || isRecepcion(),

    // Editar cliente: solo admin y recepción
    canEditCliente: isAdmin() || isRecepcion(),

    // Ver clientes: todos los roles pueden ver
    canViewClientes: true,

    // Ver historial completo: todos pueden ver
    canViewHistorial: true,

    // Información del rol actual para mostrar mensajes apropiados
    userRole,
  };
}

/**
 * Hook especializado para permisos específicos de gestión de piezas
 */
export function usePiezasPermissions() {
  const { userRole, hasPermission } = usePermissions();

  return {
    // Ver piezas: solo admin y técnico
    canViewPiezas: hasPermission("canManageParts"),

    // Crear pieza: solo admin y técnico
    canCreatePieza: hasPermission("canManageParts"),

    // Editar pieza: solo admin y técnico
    canEditPieza: hasPermission("canManageParts"),

    // Eliminar pieza: solo admin y técnico
    canDeletePieza: hasPermission("canManageParts"),

    // Información del rol actual para mostrar mensajes apropiados
    userRole,
  };
}

/**
 * Hook especializado para permisos específicos de órdenes de trabajo
 */
export function useOrdenTrabajoPermissions() {
  const { userRole, isAdmin, isTecnico, isRecepcion } = usePermissions();

  return {
    // Crear/editar órdenes de trabajo: todos los roles pueden
    canCreateOrden: true,
    canEditOrden: true,

    // Eliminar órdenes: solo admin y técnico
    canDeleteOrden: isAdmin() || isTecnico(),

    // Crear cotizaciones: solo admin y técnico
    canCreateCotizacion: isAdmin() || isTecnico(),

    // Crear informes: solo admin y técnico
    canCreateInforme: isAdmin() || isTecnico(),

    // Ver cotizaciones/informes existentes: todos pueden ver si existen
    canViewCotizacion: true,
    canViewInforme: true,

    // Editar cotizaciones/informes: solo admin y técnico
    canEditCotizacion: isAdmin() || isTecnico(),
    canEditInforme: isAdmin() || isTecnico(),

    // Aprobar/rechazar cotizaciones: recepción, admin y técnico
    canApproveCotizacion: true, // Todos los roles pueden aprobar/rechazar

    // Registrar salida de equipo: recepcionista, técnico y administrador
    canRegistrarSalida: isRecepcion() || isTecnico() || isAdmin(),

    // Función para determinar qué botones debe ver según el estado de la orden
    getVisibleActions: (orden: {
      cotizacion_id?: number;
      informe_id?: number;
      estado?: string;
    }) => {
      // Estados que permiten registro de salida
      const estadosCompatiblesSalida = [
        "espera_de_retiro",
        "entregado",
        "abandonado",
        "equipo_no_reparable",
      ];

      return {
        // Botones de cotización
        showCreateCotizacion:
          !orden.cotizacion_id && (isAdmin() || isTecnico()),
        showViewCotizacion: !!orden.cotizacion_id,

        // Botones de informe
        showCreateInforme: !orden.informe_id && (isAdmin() || isTecnico()),
        showViewInforme: !!orden.informe_id,

        // Botón de eliminar orden
        showDeleteOrden: isAdmin() || isTecnico(),

        // Botón de registrar salida
        showRegistrarSalida:
          (isRecepcion() || isTecnico() || isAdmin()) &&
          estadosCompatiblesSalida.includes(orden.estado || ""),
      };
    },

    // Función para determinar acciones disponibles en cotizaciones según el estado y rol
    getCotizacionActions: (cotizacion: {
      is_aprobada?: boolean;
      is_borrador?: boolean;
      estado_orden?: string;
    }) => {
      const isAdmin = userRole === "admin";
      const isTecnico = userRole === "tecnico";
      const isRecepcion = userRole === "recepcion";

      return {
        // Crear/editar cotización: solo admin y técnico
        canCreate: isAdmin || isTecnico,
        canEdit: isAdmin || isTecnico,

        // Ver cotización: todos pueden ver
        canView: true,

        // Aprobar/rechazar: recepción puede hacerlo si la cotización está enviada (no es borrador)
        canApprove:
          !cotizacion.is_borrador && (isRecepcion || isAdmin || isTecnico),

        // Solo lectura para recepción en borradores y cotizaciones no enviadas
        isReadOnly: isRecepcion && (cotizacion.is_borrador || false),
      };
    },

    // Información del rol actual
    userRole,
    isRecepcion: isRecepcion(),
  };
}
