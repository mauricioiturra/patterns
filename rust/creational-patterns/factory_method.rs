use std::fmt;

// Interfaz de notificación
trait Notification {
    fn send(&self, message: &str);
}

// Implementación concreta de notificación por correo electrónico
struct EmailNotification;

impl Notification for EmailNotification {
    fn send(&self, message: &str) {
        println!("Enviando correo electrónico: {}", message);
    }
}

// Implementación concreta de notificación por SMS
struct SMSNotification;

impl Notification for SMSNotification {
    fn send(&self, message: &str) {
        println!("Enviando SMS: {}", message);
    }
}

// Implementación concreta de notificación push
struct PushNotification;

impl Notification for PushNotification {
    fn send(&self, message: &str) {
        println!("Enviando notificación push: {}", message);
    }
}

// Enumeración de tipos de notificación
#[derive(Debug)]
enum NotificationType {
    Email,
    Sms,
    Push,
}

// Error personalizado para tipos de notificación no válidos
#[derive(Debug)]
struct InvalidNotificationTypeError;

impl fmt::Display for InvalidNotificationTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tipo de notificación no válido")
    }
}

// Función de fábrica de notificaciones
fn create_notification(notification_type: NotificationType) -> Result<Box<dyn Notification>, InvalidNotificationTypeError> {
    match notification_type {
        NotificationType::Email => Ok(Box::new(EmailNotification)),
        NotificationType::Sms => Ok(Box::new(SMSNotification)),
        NotificationType::Push => Ok(Box::new(PushNotification)),
    }
}

fn main() {
    // Crear una notificación por correo electrónico
    let email_notification = create_notification(NotificationType::Email).unwrap();
    email_notification.send("¡Hola por correo electrónico!");

    // Crear una notificación por SMS
    let sms_notification = create_notification(NotificationType::Sms).unwrap();
    sms_notification.send("¡Hola por SMS!");

    // Crear una notificación push
    let push_notification = create_notification(NotificationType::Push).unwrap();
    push_notification.send("¡Hola por notificación push!");
}
