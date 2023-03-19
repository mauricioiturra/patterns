from abc import ABC, abstractmethod

# Interfaz de notificación
class Notification(ABC):
    @abstractmethod
    def send(self, message: str):
        pass

# Implementación concreta de notificación por correo electrónico
class EmailNotification(Notification):
    def send(self, message: str):
        print(f"Enviando correo electrónico: {message}")

# Implementación concreta de notificación por SMS
class SMSNotification(Notification):
    def send(self, message: str):
        print(f"Enviando SMS: {message}")

# Implementación concreta de notificación push
class PushNotification(Notification):
    def send(self, message: str):
        print(f"Enviando notificación push: {message}")

# Enumeración de tipos de notificación
class NotificationType:
    EMAIL = "email"
    SMS = "sms"
    PUSH = "push"

# Función de fábrica de notificaciones
def create_notification(notification_type: str) -> Notification:
    if notification_type == NotificationType.EMAIL:
        return EmailNotification()
    elif notification_type == NotificationType.SMS:
        return SMSNotification()
    elif notification_type == NotificationType.PUSH:
        return PushNotification()
    else:
        raise ValueError("Tipo de notificación no válido")

def main():
    # Crear una notificación por correo electrónico
    email_notification = create_notification(NotificationType.EMAIL)
    email_notification.send("¡Hola por correo electrónico!")

    # Crear una notificación por SMS
    sms_notification = create_notification(NotificationType.SMS)
    sms_notification.send("¡Hola por SMS!")

    # Crear una notificación push
    push_notification = create_notification(NotificationType.PUSH)
    push_notification.send("¡Hola por notificación push!")

if __name__ == "__main__":
    main()
