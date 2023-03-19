interface Notification {
    send(message: string): void;
}

class EmailNotification implements Notification {
    send(message: string): void {
        console.log(`Enviando correo electrónico: ${message}`);
    }
}

class SMSNotification implements Notification {
    send(message: string): void {
        console.log(`Enviando SMS: ${message}`);
    }
}

class PushNotification implements Notification {
    send(message: string): void {
        console.log(`Enviando notificación push: ${message}`);
    }
}

enum NotificationType {
    EMAIL = "email",
    SMS = "sms",
    PUSH = "push",
}

function createNotification(type: NotificationType): Notification {
    switch (type) {
        case NotificationType.EMAIL:
            return new EmailNotification();
        case NotificationType.SMS:
            return new SMSNotification();
        case NotificationType.PUSH:
            return new PushNotification();
        default:
            throw new Error("Tipo de notificación no válido");
    }
}

function main() {
    const emailNotification = createNotification(NotificationType.EMAIL);
    emailNotification.send("¡Hola por correo electrónico!");

    const smsNotification = createNotification(NotificationType.SMS);
    smsNotification.send("¡Hola por SMS!");

    const pushNotification = createNotification(NotificationType.PUSH);
    pushNotification.send("¡Hola por notificación push!");
}

main();
