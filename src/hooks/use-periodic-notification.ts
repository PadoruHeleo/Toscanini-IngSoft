import { useEffect } from 'react';
import { useToastContext } from '@/contexts/ToastContext';
import { useView } from '@/contexts/ViewContext';
import { useAuth } from '@/contexts/AuthContext';
import { invoke } from "@tauri-apps/api/core";

interface OrdenTrabajoDetallada {
    // Campos básicos de la orden
    orden_id: number;
    orden_codigo?: string;
    orden_desc?: string;
    prioridad?: string;
    estado?: string;
    has_garantia?: boolean;
    equipo_id?: number;
    created_by?: number;
    cotizacion_id?: number;
    informe_id?: number;
    pre_informe?: string;
    created_at?: string;
    finished_at?: string;
    // Campos del equipo 
    numero_serie?: string;
    equipo_marca?: string;
    equipo_modelo?: string;
    equipo_tipo?: string;
    // Campos del cliente 
    cliente_id?: number;
    cliente_nombre?: string;
    // Campo del usuario creador
    creador_nombre?: string;
    // Campos de la cotización
    cotizacion_codigo?: string;
    costo_total?: number;
    // Campo del informe
    informe_codigo?: string;
}

// Funcion para las ordenes sin cotizacion de ningun tipo por dos dias
async function checkOrdenSinCotizacion(ordenesData:OrdenTrabajoDetallada[]) {
    const now = new Date();
    let hasOldOrders: boolean = false;
    let messages : Array<string> = [];

    for (const orden of ordenesData) {
        if (orden.cotizacion_id === null && orden.created_at) {
            const createdAt = new Date(orden.created_at);
            const diffTime = Math.abs(now.getTime() - createdAt.getTime());
            const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

            if (diffDays >= 2) {
                hasOldOrders = true;
                let message : string = `La orden ${orden.orden_codigo} del equipo ${orden.equipo_marca} ${orden.equipo_modelo}, ingresado el ${createdAt.toLocaleDateString()}, lleva ${diffDays} días sin cotización`;
                messages.push(message);
            }
        }
    }
    // Estas funciones retornan si es que tienen ordenes que avisar, el tipo y el mensaje prehecho
    return {
        hasOldOrders: hasOldOrders,
        type: "sin cotización",
        messages: messages,
    };
}

// Funcion para las ordens que llevan mas de 3 dias sin cambiar de recibido
async function checkOrdenCotNoEnviada(ordenesData:OrdenTrabajoDetallada[]) {
    const now = new Date();
    let hasOldOrders: boolean = false;
    let messages : Array<string> = [];

    for (const orden of ordenesData) {
        if (orden.estado == "recibido" && orden.created_at) {
            const createdAt = new Date(orden.created_at);
            const diffTime = Math.abs(now.getTime() - createdAt.getTime());
            const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));
            
            if (diffDays >= 3) {
                hasOldOrders = true;
                let message : string = `La orden ${orden.orden_codigo} del equipo ${orden.equipo_marca} ${orden.equipo_modelo}, ingresado el ${createdAt.toLocaleDateString()}, lleva ${diffDays} días sin cotización enviada al cliente`;
                messages.push(message);
            }
        }
    }

    return {
        hasOldOrders: hasOldOrders,
        type: "con cotización no enviada",
        messages: messages,
    };
}

// Funcion para la orden de prioridad alta no atendidas
async function checkOrdenPrioridadNoAtendida(ordenesData:OrdenTrabajoDetallada[]) {
    const now = new Date();
    let hasOldOrders: boolean = false;
    let messages : Array<string> = [];

    for (const orden of ordenesData) {
        if (orden.estado == "recibido" && orden.created_at && orden.prioridad == "alta") {
            const createdAt = new Date(orden.created_at);
            const diffTime = Math.abs(now.getTime() - createdAt.getTime());
            const diffHours = Math.floor(diffTime / (1000 * 60 * 60));

            if (diffHours >= 24) {
                hasOldOrders = true;
                let message : string = `La orden de prioridad ${orden.prioridad} ${orden.orden_codigo} del equipo ${orden.equipo_marca} ${orden.equipo_modelo}, ingresado el ${createdAt.toLocaleDateString()}, lleva ${diffHours} horas sin ser atendida`;
                messages.push(message);
            }
        }
    }

    return {
        hasOldOrders: hasOldOrders,
        type: "con prioridad alta no atendida",
        messages: messages,
    };
}

// Funcion que llama a todas las posibles notifiaciones
async function checkOrdenesAllNotifications() {
    const ordenesData = await invoke<OrdenTrabajoDetallada[]>("get_ordenes_trabajo_detalladas");
    const result_sin_cotizacion = await checkOrdenSinCotizacion(ordenesData);
    const result_cot_no_enviada = await checkOrdenCotNoEnviada(ordenesData);
    const result_prioridad_no_atendida = await checkOrdenPrioridadNoAtendida(ordenesData);

    return [result_sin_cotizacion,result_cot_no_enviada, result_prioridad_no_atendida];
};

// Funcion para las notificaciones del tecnico, se ejecuta cuando carga en la view
async function notificacionesTecnico(minutes: number) {
    const { info } = useToastContext();
    useEffect(() => {
        const checkAndNotify = async () => {
            // Llamada a todas las notificaciones posibles para el tecnico
            const results = await checkOrdenesAllNotifications();
            // Iterando entre los resultados y creando los mensajes
            for (const result of results) {
                if (result.hasOldOrders) {
                    let title: string = `⚠️ Órden ${result.type}`;
                    for (const message of result.messages) {
                        info(
                            title,
                            message
                        );
                    }
                }
            }
        };
        // Llamada inicial para las notifiaciones
        checkAndNotify();
        // Intervalo de notificaciones
        const intervalId = setInterval(checkAndNotify, minutes * 60 * 1000);
        return () => {
            clearInterval(intervalId);
        };
    }, [info, minutes]);
}

// Funcion que se llama en el App, repitiendose con el tiempo del parametro
export function usePeriodicNotification(intervalMinutes: number = 5) {
    const { currentView } = useView();
    const { user } = useAuth();

    // Evitar notificaciones en el login
    if (currentView == "login"){
        return null;
    }
    // Notificaciones para técnico
    if (user?.usuario_rol == "tecnico" || user?.usuario_rol == "admin"){
        notificacionesTecnico(intervalMinutes);
    }
}
