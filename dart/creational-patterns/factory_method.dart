// Interfaz de notificación
abstract class Notification {
  void send(String message);
}

// Implementación concreta de notificación por correo electrónico
class EmailNotification implements Notification {
  @override
  void send(String message) {
    print('Enviando correo electrónico: $message');
  }
}

// Implementación concreta de notificación por SMS
class SMSNotification implements Notification {
  @override
  void send(String message) {
    print('Enviando SMS: $message');
  }
}

// Implementación concreta de notificación push
class PushNotification implements Notification {
  @override
  void send(String message) {
    print('Enviando notificación push: $message');
  }
}

// Enumeración de tipos de notificación
enum NotificationType { email, sms, push }

// Función de fábrica de notificaciones
Notification createNotification(NotificationType type) {
  switch (type) {
    case NotificationType.email:
      return EmailNotification();
    case NotificationType.sms:
      return SMSNotification();
    case NotificationType.push:
      return PushNotification();
    default:
      throw ArgumentError('Tipo de notificación no válido');
  }
}

void main() {
  // Crear una notificación por correo electrónico
  final emailNotification = createNotification(NotificationType.email);
  emailNotification.send('¡Hola por correo electrónico!');

  // Crear una notificación por SMS
  final smsNotification = createNotification(NotificationType.sms);
  smsNotification.send('¡Hola por SMS!');

  // Crear una notificación push
  final pushNotification = createNotification(NotificationType.push);
  pushNotification.send('¡Hola por notificación push!');
}
