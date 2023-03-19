#include <iostream>
#include <memory>

// Interfaz de notificación
class Notification {
public:
    virtual ~Notification() = default;
    virtual void send(const std::string& message) = 0;
};

// Implementación concreta de notificación por correo electrónico
class EmailNotification : public Notification {
public:
    void send(const std::string& message) override {
        std::cout << "Enviando correo electrónico: " << message << std::endl;
    }
};

// Implementación concreta de notificación por SMS
class SMSNotification : public Notification {
public:
    void send(const std::string& message) override {
        std::cout << "Enviando SMS: " << message << std::endl;
    }
};

// Implementación concreta de notificación push
class PushNotification : public Notification {
public:
    void send(const std::string& message) override {
        std::cout << "Enviando notificación push: " << message << std::endl;
    }
};

// Enumeración de tipos de notificación
enum class NotificationType {
    EMAIL,
    SMS,
    PUSH
};

// Función de fábrica de notificaciones
std::unique_ptr<Notification> createNotification(NotificationType type) {
    switch (type) {
        case NotificationType::EMAIL:
            return std::make_unique<EmailNotification>();
        case NotificationType::SMS:
            return std::make_unique<SMSNotification>();
        case NotificationType::PUSH:
            return std::make_unique<PushNotification>();
        default:
            throw std::runtime_error("Tipo de notificación no válido");
    }
}

int main() {
    // Crear una notificación por correo electrónico
    auto emailNotification = createNotification(NotificationType::EMAIL);
    emailNotification->send("¡Hola por correo electrónico!");

    // Crear una notificación por SMS
    auto smsNotification = createNotification(NotificationType::SMS);
    smsNotification->send("¡Hola por SMS!");

    // Crear una notificación push
    auto pushNotification = createNotification(NotificationType::PUSH);
    pushNotification->send("¡Hola por notificación push!");

    return 0;
}
