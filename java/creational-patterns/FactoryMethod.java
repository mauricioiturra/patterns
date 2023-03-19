// Interfaz de notificación
public interface Notification {
    void send(String message);
}

// Implementación concreta de notificación por correo electrónico
class EmailNotification implements Notification {
    @Override
    public void send(String message) {
        System.out.println("Enviando correo electrónico: " + message);
    }
}

// Implementación concreta de notificación por SMS
class SMSNotification implements Notification {
    @Override
    public void send(String message) {
        System.out.println("Enviando SMS: " + message);
    }
}

// Implementación concreta de notificación push
class PushNotification implements Notification {
    @Override
    public void send(String message) {
        System.out.println("Enviando notificación push: " + message);
    }
}

// Enumeración de tipos de notificación
enum NotificationType {
    EMAIL, SMS, PUSH
}

// Clase de fábrica de notificaciones
class NotificationFactory {
    public static Notification createNotification(NotificationType type) {
        switch (type) {
            case EMAIL:
                return new EmailNotification();
            case SMS:
                return new SMSNotification();
            case PUSH:
                return new PushNotification();
            default:
                throw new IllegalArgumentException("Tipo de notificación no válido");
        }
    }
}

// Clase principal para probar el patrón de diseño Método de Fábrica
public class Main {
    public static void main(String[] args) {
        // Crear una notificación por correo electrónico
        Notification emailNotification = NotificationFactory.createNotification(NotificationType.EMAIL);
        emailNotification.send("¡Hola por correo electrónico!");

        // Crear una notificación por SMS
        Notification smsNotification = NotificationFactory.createNotification(NotificationType.SMS);
        smsNotification.send("¡Hola por SMS!");

        // Crear una notificación push
        Notification pushNotification = NotificationFactory.createNotification(NotificationType.PUSH);
        pushNotification.send("¡Hola por notificación push!");
    }
}
