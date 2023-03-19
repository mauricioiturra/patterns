### Interfaz de notificación
Método enviar(mensaje)

### Clase EmailNotification implementa notificación
Método enviar(mensaje):
    Escribir "Enviando correo electrónico: " + mensaje

### Clase SMSNotification implementa notificación
Método enviar(mensaje):
    Escribir "Enviando SMS: " + mensaje

### Clase PushNotification implementa notificación
Método enviar(mensaje):
    Escribir "Enviando notificación push: " + mensaje

### Enumeración NotificationType
EMAIL, SMS, PUSH

### Función createNotification(tipo_de_notificación)
Si tipo_de_notificación es igual a EMAIL:
    Retornar una nueva instancia de EmailNotification
Si tipo_de_notificación es igual a SMS:
    Retornar una nueva instancia de SMSNotification
Si tipo_de_notificación es igual a PUSH:
    Retornar una nueva instancia de PushNotification
Sino:
    Lanzar un error "Tipo de notificación no válido"

### Función principal
emailNotification <- createNotification(EMAIL)
emailNotification.enviar("¡Hola por correo electrónico!")

smsNotification <- createNotification(SMS)
smsNotification.enviar("¡Hola por SMS!")

pushNotification <- createNotification(PUSH)
pushNotification.enviar("¡Hola por notificación push!")
